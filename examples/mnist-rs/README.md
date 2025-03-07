# mnist-rs

This demo demonstrates how to train and perform inference in TEE.

## Install the TA

There are two TAs in the project:

| TA | UUID | Usage |
| ---- | ---- | ---- |
| Train | 1b5f5b74-e9cf-4e62-8c3e-7e41da6d76f6 | for training new Model|
| Inference | ff09aa8a-fbb9-4734-ae8c-d7cd1a3f6744 | for performing reference|

Separate them as normally training consumes much more resource than performing
inference, which results in different build settings(memory, concurrency, etc).

Make sure to install them before attempting to use any functions, you can refer
to the [Makefile](../../Makefile) in the root directory for details.

## Running the Host

There are three subcommands in the host:

1. Train

    Trains a new model and exports it to the given path.

    ``` shell
    mnist-rs train -o model.bin
    ```

    This subcommand downloads the MNIST dataset, feeds the dataset into TEE and
    perform training inside TEE, outputs the model to the given path after the
    training finished.

    ```mermaid
    sequenceDiagram
    actor D as Developer
    participant C as mnist-rs(REE)
    participant T as Train TA(TEE)
    
    D ->> C: train -o model.bin
    C ->> C: Fetch mnist dataset

    C ->> T: open_session_with_operation
    T ->> T: Initialize Global Trainer<br/> with given learning rate
    T ->> C: Initialize finished

    loop iterate over num_epoches
        rect rgb(191, 223, 255)
        note right of C: Train
            loop chunk by batch_size over train datasets
                C ->> T: invoke_command Train with chunk datasets
                T ->> T: Forward with given data
                T ->> T: Backward Optimization
                T ->> C: Train Output(loss, accuracy)
            end
        end
        rect rgb(200, 150, 255)
        note right of C: Valid
            loop chunk by batch_size over test datasets
                C ->> T: invoke_command Test with chunk datasets
                T ->> T: Forward with given data
                T ->> C: Test Output(loss, accuracy)
            end
        end
    end

    C ->> T: Export Command
    T ->> C: Model Record
    C ->> D: model.bin
    ```

    For detailed usage, run: `mnist-rs train --help`, a demo output is:

    ``` shell
    Usage: mnist-rs train [OPTIONS]

    Options:
      -n, --num-epochs <NUM_EPOCHS>        [default: 6]
      -b, --batch-size <BATCH_SIZE>        [default: 64]
      -l, --learning-rate <LEARNING_RATE>  [default: 0.0001]
      -o, --output <OUTPUT>
      -h, --help                           Print help
    ```

2. Infer

    Loads a model from the given path, tests it with a given image, and prints
    the inference result.

    ```shell
    mnist-rs infer -m model.bin -b samples/7.bin -i samples/7.png
    ```

    This subcommand loads the model the model from the given path and tests it
    with the given binaries and images, and prints the inference results. For
    convenience, you can use the sample binaries and images in the `samples`
    folder.

    ```mermaid
    sequenceDiagram
    actor D as Developer
    participant C as mnist-rs(REE)
    participant T as Inference TA(TEE)
    
    D ->> C: infer -m model.bin<br/> -b samples/7.bin<br/> -i samples/7.png

    C ->> C: load Model Record from disk
    C ->> T: open_session_with_operation
    T ->> T: Initialize Global Model<br/> with given Model Record
    T ->> C: Initialize finished

    rect rgb(191, 223, 255)
    note right of C: Infer with samples/7.bin
        C ->> C: Load file from disk.
        C ->> T: invoke_command: Feed data
        T ->> T: Forward with given data
        T ->> C: Infer result
        C ->> D: Print result
    end

    rect rgb(191, 223, 255)
    note right of C: Infer with samples/7.png
        C ->> C: Load image from disk.
        C ->> C: Convert image to luma8 binary
        C ->> T: invoke_command with data
        T ->> T: Forward with given data
        T ->> C: Infer result
        C ->> D: Print result
    end

    ```
    For detailed usage, run: `mnist-rs infer --help`, a demo output is:

    ```shell
    Usage: mnist-rs infer [OPTIONS] --model <MODEL>

    Options:
        -m, --model <MODEL>    The path of the model
        -b, --binary <BINARY>  The path of the input binary, must be 768 byte binary, can be multiple
        -i, --image <IMAGE>    The path of the input image, must be dimension of 28x28, can be multiple
        -h, --help             Print help
    ```

3. Serve

    Loads a model from the given path, starts a web server and serves it as an
    API.

    ```shell
    mnist-rs serve -m model.bin
    ```

    This subcommand loads the model the model from the given path and starts a
    web server to provide inference APIs.

    **Available APIs**:

    | Method | Endpoint | Body |
    | ---- | ---- | ---- |
    | POST | `/inference/image` | an image with dimensions 28x28 |
    | POST | `/inference/binary` | a 784-byte binary |

    You can test the server with the following commands:

    ```shell
    # Perform inference using an image
    curl --data-binary "@./samples/7.png" http://localhost:3000/inference/image
    # Perform inference using a binary file
    curl --data-binary "@./samples/7.bin" http://localhost:3000/inference/binary
    ```

    ```mermaid
    sequenceDiagram
    actor D as Developer
    actor H as HttpClient
    participant C as mnist-rs(REE)
    participant T as Inference TA(TEE)
    
    D ->> C: serve -m model.bin

    C ->> C: Load Model Record from disk
    C ->> T: open_session_with_operation
    T ->> T: Initialize Global Model<br/> with given Model Record
    T ->> C: Initialize finished

    C ->> C: Start http server

    loop accept request
        par /inference/binary
            H ->> C: Request with binary data
            C ->> T: invoke_command: Feed data
            T ->> T: Forward with given data
            T ->> C: Infer result
            C ->> H: Infer result
        end
        par /inference/image
            H ->> C: Request with image data
            C ->> C: Convert image to luma8 binary
            C ->> T: invoke_command: Feed data
            T ->> T: Forward with given data
            T ->> C: Infer result
            C ->> H: Infer result
        end
    end
    

    ```

    For detailed usage, run: `mnist-rs serve --help`, a demo output is:

    ```shell
    Usage: mnist-rs serve [OPTIONS] --model <MODEL>

    Options:
        -m, --model <MODEL>  The path of the model
        -p, --port <PORT>    [default: 3000]
        -h, --help           Print help
    ```

## Credits

This demo project is inspired by the crates and examples from
[tracel-ai/burn](https://github.com/tracel-ai/burn), including:

1. [crates/burn-no-std-tests](https://github.com/tracel-ai/burn/tree/v0.16.0/crates/burn-dataset)
2. [examples/custom-training-loop](https://github.com/tracel-ai/burn/tree/v0.16.0/examples/custom-training-loop)
3. [examples/mnist-inference-web](https://github.com/tracel-ai/burn/tree/v0.16.0/examples/mnist-inference-web)

Special thanks to @[Guillaume Lagrange](https://github.com/laggui) for sharing
knowledge and providing early reviews.

TODO: standard license files after 0.4.0 released.
