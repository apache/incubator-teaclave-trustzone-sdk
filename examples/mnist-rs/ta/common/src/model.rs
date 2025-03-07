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
    prelude::*,
    record::{FullPrecisionSettings, Recorder, RecorderError},
};
use proto::{Image, IMAGE_SIZE, NUM_CLASSES};

/// This is a simple model designed solely to demonstrate how to train and
/// perform inference, don't use it in production.
#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    linear: nn::Linear<B>,
}

impl<B: Backend> Model<B> {
    pub fn new(device: &B::Device) -> Self {
        Self {
            linear: nn::LinearConfig::new(IMAGE_SIZE, NUM_CLASSES).init(device),
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        self.linear.forward(input)
    }

    pub fn export(&self) -> Result<Vec<u8>, RecorderError> {
        let recorder = burn::record::BinBytesRecorder::<FullPrecisionSettings>::new();
        recorder.record(self.clone().into_record(), ())
    }

    pub fn import(device: &B::Device, record: Vec<u8>) -> Result<Self, RecorderError> {
        let recorder = burn::record::BinBytesRecorder::<FullPrecisionSettings>::new();
        let record = recorder.load(record, device)?;

        let m = Self::new(device);
        Ok(m.load_record(record))
    }
}

impl<B: Backend> Model<B> {
    pub fn image_to_tensor(device: &B::Device, image: &Image) -> Tensor<B, 2> {
        let tensor = TensorData::from(image.as_slice()).convert::<B::FloatElem>();
        let tensor = Tensor::<B, 1>::from_data(tensor, device);
        let tensor = tensor.reshape([1, IMAGE_SIZE]);

        // Normalize input: make between [0,1] and make the mean=0 and std=1
        // values mean=0.1307,std=0.3081 were copied from Pytorch Mist Example
        // https://github.com/pytorch/examples/blob/54f4572509891883a947411fd7239237dd2a39c3/mnist/main.py#L122
        ((tensor / 255) - 0.1307) / 0.3081
    }

    pub fn images_to_tensors(device: &B::Device, images: &[Image]) -> Tensor<B, 2> {
        let tensors = images
            .iter()
            .map(|v| Self::image_to_tensor(device, v))
            .collect();
        Tensor::cat(tensors, 0)
    }

    pub fn labels_to_tensors(device: &B::Device, labels: &[u8]) -> Tensor<B, 1, Int> {
        let targets = labels
            .iter()
            .map(|item| {
                Tensor::<B, 1, Int>::from_data([(*item as i64).elem::<B::IntElem>()], device)
            })
            .collect();
        Tensor::cat(targets, 0)
    }
}
