// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use alloc::vec::Vec;
use burn::{
    module::AutodiffModule,
    nn::loss::CrossEntropyLoss,
    optim::{adaptor::OptimizerAdaptor, Adam, AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    record::RecorderError,
    tensor::{backend::AutodiffBackend, cast::ToElement},
};
use common::Model;
use proto::{train::Output, Image};

pub struct Trainer<B: AutodiffBackend> {
    model: Model<B>,
    device: B::Device,
    optim: OptimizerAdaptor<Adam, Model<B>, B>,
    lr: f64,
}

impl<B: AutodiffBackend> Trainer<B> {
    pub fn new(device: B::Device, lr: f64) -> Self {
        let mut seed = [0_u8; 8];
        optee_utee::Random::generate(seed.as_mut_slice());
        B::seed(u64::from_le_bytes(seed));

        Self {
            optim: AdamConfig::new().init(),
            model: Model::new(&device),
            device,
            lr,
        }
    }

    // Originally inspired by the burn/examples/custom-training-loop package.
    // You may refer to
    // https://github.com/tracel-ai/burn/blob/v0.16.0/examples/custom-training-loop
    // for details.
    pub fn train(&mut self, images: &[Image], labels: &[u8]) -> Output {
        let images = Model::images_to_tensors(&self.device, images);
        let targets = Model::labels_to_tensors(&self.device, labels);
        let model = self.model.clone();

        let output = model.forward(images);
        let loss =
            CrossEntropyLoss::new(None, &output.device()).forward(output.clone(), targets.clone());
        let accuracy = accuracy(output, targets);

        // Gradients for the current backward pass
        let grads = loss.backward();
        // Gradients linked to each parameter of the model.
        let grads = GradientsParams::from_grads(grads, &model);
        // Update the model using the optimizer.
        self.model = self.optim.step(self.lr, model, grads);

        Output {
            loss: loss.into_scalar().to_f32(),
            accuracy,
        }
    }

    // Originally inspired by the burn/examples/custom-training-loop package.
    // You may refer to
    // https://github.com/tracel-ai/burn/blob/v0.16.0/examples/custom-training-loop
    // for details.
    pub fn valid(&self, images: &[Image], labels: &[u8]) -> Output {
        // Get the model without autodiff.
        let model_valid = self.model.valid();

        let images = Model::images_to_tensors(&self.device, images);
        let targets = Model::labels_to_tensors(&self.device, labels);

        let output = model_valid.forward(images);
        let loss =
            CrossEntropyLoss::new(None, &output.device()).forward(output.clone(), targets.clone());
        let accuracy = accuracy(output, targets);

        Output {
            loss: loss.into_scalar().to_f32(),
            accuracy,
        }
    }

    pub fn export(&self) -> Result<Vec<u8>, RecorderError> {
        self.model.export()
    }
}

// Originally copied from the burn/crates/no-std-tests package. You may refer
// to https://github.com/tracel-ai/burn/blob/v0.16.0/crates/burn-no-std-test for
// details.
fn accuracy<B: Backend>(output: Tensor<B, 2>, targets: Tensor<B, 1, Int>) -> f32 {
    let predictions = output.argmax(1).squeeze(1);
    let num_predictions: usize = targets.dims().iter().product();
    let num_corrects = predictions.equal(targets).int().sum().into_scalar();

    num_corrects.elem::<f32>() / num_predictions as f32 * 100.0
}
