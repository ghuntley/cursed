# CURSED Neural Network Module (nnz)

## Overview

The **nnz** module provides a comprehensive neural network framework for deep learning in CURSED. It offers a complete toolkit for building, training, and deploying neural networks with support for various architectures, optimizers, and advanced training techniques.

## Features

### Core Components
- **Neural Network Architecture**: Flexible layer-based network construction
- **Layer Types**: Dense, Convolutional, Pooling, Dropout, Batch Normalization
- **Activation Functions**: ReLU, Sigmoid, Tanh, Softmax, Leaky ReLU, Swish, GELU, ELU, SELU
- **Loss Functions**: MSE, MAE, Binary/Categorical Cross-entropy, Huber Loss
- **Optimizers**: SGD, Momentum, Adam, RMSprop with learning rate scheduling

### Advanced Features
- **Regularization**: L1/L2 regularization, Dropout, Early stopping
- **Training Techniques**: Batch training, Gradient clipping, Learning rate scheduling
- **Ensemble Methods**: Model averaging, Weighted predictions
- **Transfer Learning**: Fine-tuning pretrained models
- **Hyperparameter Optimization**: Grid search utilities

### Architecture Support
- **Feedforward Networks**: Multi-layer perceptrons
- **Convolutional Networks**: Basic CNN support with Conv2D and MaxPool2D
- **Autoencoders**: Unsupervised learning for dimensionality reduction
- **Classification/Regression**: Both supervised learning paradigms

## API Reference

### Neural Network Creation

```cursed
yeet "nnz"

fr fr Create a neural network
sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_ADAM())

fr fr Add layers
sus layer1 Layer = layer_create_dense(784, 128, ACTIVATION_RELU())
sus layer2 Layer = layer_create_dense(128, 10, ACTIVATION_SOFTMAX())

network = neural_network_add_layer(network, layer1)
network = neural_network_add_layer(network, layer2)
```

### Training

```cursed
fr fr Train for multiple epochs
sus epoch drip = 0
bestie (epoch < 100) {
    sus loss meal = neural_network_train_epoch(network, train_data, train_labels, 
                                               num_samples, input_size, batch_size)
    
    ready (epoch % 10 == 0) {
        sus accuracy meal = neural_network_evaluate(network, test_data, test_labels,
                                                    num_test, input_size)
        vibez.spill("Epoch ", epoch, ": Loss = ", loss, ", Accuracy = ", accuracy)
    }
    
    epoch = epoch + 1
}
```

### Layer Types

#### Dense Layer
```cursed
sus dense_layer Layer = layer_create_dense(input_size, output_size, ACTIVATION_RELU())
```

#### Dropout Layer
```cursed
sus dropout_layer Layer = layer_create_dropout(0.5)  fr fr 50% dropout rate
```

#### Batch Normalization
```cursed
sus bn_layer Layer = layer_create_batch_norm(feature_size)
```

### Activation Functions

```cursed
fr fr Available activation types
ACTIVATION_RELU()      fr fr Rectified Linear Unit
ACTIVATION_SIGMOID()   fr fr Sigmoid function
ACTIVATION_TANH()      fr fr Hyperbolic tangent
ACTIVATION_SOFTMAX()   fr fr Softmax (for classification)
ACTIVATION_LEAKY_RELU() fr fr Leaky ReLU
ACTIVATION_SWISH()     fr fr Swish activation

fr fr Advanced activations
sus output meal = swish(input)           fr fr Swish: x * sigmoid(x)
sus output meal = gelu(input)            fr fr GELU approximation
sus output meal = elu(input, 1.0)        fr fr ELU with alpha=1.0
sus output meal = selu(input)            fr fr Self-normalizing ELU
```

### Loss Functions

```cursed
fr fr Regression losses
sus mse meal = mse_loss(predictions, targets)
sus mae meal = mae_loss(predictions, targets)
sus huber meal = huber_loss(predictions, targets, 1.0)

fr fr Classification losses
sus binary_ce meal = binary_crossentropy_loss(predictions, targets)
sus categorical_ce meal = categorical_crossentropy_loss(predictions, targets)
sus sparse_ce meal = sparse_categorical_crossentropy_loss(predictions, target_class)
```

### Optimizers

```cursed
fr fr Available optimizers
OPTIMIZER_SGD()        fr fr Stochastic Gradient Descent
OPTIMIZER_MOMENTUM()   fr fr SGD with momentum
OPTIMIZER_ADAM()       fr fr Adam optimizer
OPTIMIZER_RMSPROP()    fr fr RMSprop optimizer

fr fr Create network with specific optimizer
sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_ADAM())
```

### Regularization

```cursed
fr fr L1/L2 regularization
sus l1_loss meal = l1_regularization_loss(weights, 0.01)
sus l2_loss meal = l2_regularization_loss(weights, 0.01)
sus elastic_loss meal = elastic_net_regularization_loss(weights, 0.01, 0.01)

fr fr Early stopping
sus should_stop lit = early_stopping_check(validation_losses, patience, min_delta)

fr fr Gradient clipping
sus clipped_gradients []meal = gradient_clipping(gradients, max_norm)
```

### Learning Rate Scheduling

```cursed
fr fr Step decay
sus new_lr meal = learning_rate_step_decay(initial_lr, epoch, step_size, gamma)

fr fr Exponential decay
sus new_lr meal = learning_rate_exponential_decay(initial_lr, epoch, decay_rate)

fr fr Cosine annealing
sus new_lr meal = learning_rate_cosine_annealing(initial_lr, epoch, max_epochs)
```

### Convolutional Networks

```cursed
fr fr 2D Convolution
sus conv_output []meal = conv2d_forward(input, weights, biases, 
                                        height, width, channels,
                                        kernel_size, output_channels,
                                        stride, padding)

fr fr Max Pooling
sus pool_output []meal = maxpool2d_forward(input, height, width, channels,
                                           pool_size, stride)
```

### Ensemble Methods

```cursed
fr fr Average ensemble predictions
sus ensemble_pred []meal = ensemble_predict_average(predictions, num_models, output_size)

fr fr Weighted ensemble predictions
sus weighted_pred []meal = ensemble_predict_weighted(predictions, weights, 
                                                     num_models, output_size)
```

### Model Serialization

```cursed
fr fr Save model weights
sus saved_weights [][]meal = neural_network_save_weights(network)

fr fr Load model weights
network = neural_network_load_weights(network, saved_weights)
```

## Examples

### 1. Multi-class Classification

```cursed
yeet "nnz"
yeet "mlz"

slay demo_image_classification() cringe {
    fr fr Create dataset (example: 28x28 images, 10 classes)
    sus input_size drip = 784  fr fr 28*28
    sus num_classes drip = 10
    sus num_samples drip = 1000
    
    fr fr Create synthetic dataset
    sus train_data []meal = generate_random_data(num_samples, input_size)
    sus train_labels []meal = generate_random_labels(num_samples, num_classes)
    
    fr fr Create neural network
    sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_ADAM())
    
    fr fr Add layers
    sus layer1 Layer = layer_create_dense(input_size, 128, ACTIVATION_RELU())
    sus dropout1 Layer = layer_create_dropout(0.3)
    sus layer2 Layer = layer_create_dense(128, 64, ACTIVATION_RELU())
    sus dropout2 Layer = layer_create_dropout(0.3)
    sus output_layer Layer = layer_create_dense(64, num_classes, ACTIVATION_SOFTMAX())
    
    network = neural_network_add_layer(network, layer1)
    network = neural_network_add_layer(network, dropout1)
    network = neural_network_add_layer(network, layer2)
    network = neural_network_add_layer(network, dropout2)
    network = neural_network_add_layer(network, output_layer)
    
    fr fr Training loop
    sus epoch drip = 0
    bestie (epoch < 100) {
        sus loss meal = neural_network_train_epoch(network, train_data, train_labels,
                                                   num_samples, input_size, 32)
        
        ready (epoch % 10 == 0) {
            sus accuracy meal = neural_network_evaluate(network, train_data, train_labels,
                                                        num_samples, input_size)
            vibez.spill("Epoch ", epoch, ": Loss = ", loss, ", Accuracy = ", accuracy * 100.0, "%")
        }
        
        epoch = epoch + 1
    }
}
```

### 2. Autoencoder for Dimensionality Reduction

```cursed
slay demo_autoencoder() cringe {
    sus input_size drip = 100
    sus encoding_dim drip = 20
    sus num_samples drip = 500
    
    fr fr Generate random data
    sus data []meal = generate_random_data(num_samples, input_size)
    
    fr fr Create autoencoder
    sus autoencoder NeuralNetwork = neural_network_create(0.001, OPTIMIZER_ADAM())
    
    fr fr Encoder
    sus encoder1 Layer = layer_create_dense(input_size, 64, ACTIVATION_RELU())
    sus encoder2 Layer = layer_create_dense(64, 32, ACTIVATION_RELU())
    sus bottleneck Layer = layer_create_dense(32, encoding_dim, ACTIVATION_RELU())
    
    fr fr Decoder
    sus decoder1 Layer = layer_create_dense(encoding_dim, 32, ACTIVATION_RELU())
    sus decoder2 Layer = layer_create_dense(32, 64, ACTIVATION_RELU())
    sus decoder3 Layer = layer_create_dense(64, input_size, ACTIVATION_SIGMOID())
    
    autoencoder = neural_network_add_layer(autoencoder, encoder1)
    autoencoder = neural_network_add_layer(autoencoder, encoder2)
    autoencoder = neural_network_add_layer(autoencoder, bottleneck)
    autoencoder = neural_network_add_layer(autoencoder, decoder1)
    autoencoder = neural_network_add_layer(autoencoder, decoder2)
    autoencoder = neural_network_add_layer(autoencoder, decoder3)
    
    fr fr Train autoencoder (input = target)
    demo_deep_autoencoder(data, num_samples, input_size, encoding_dim, 200)
}
```

### 3. Convolutional Neural Network

```cursed
slay demo_simple_cnn() cringe {
    sus image_height drip = 32
    sus image_width drip = 32
    sus image_channels drip = 3
    sus num_classes drip = 10
    
    fr fr Flatten image for demonstration
    sus input_size drip = image_height * image_width * image_channels
    sus num_samples drip = 100
    
    sus train_data []meal = generate_random_data(num_samples, input_size)
    sus train_labels []meal = generate_random_labels(num_samples, num_classes)
    
    fr fr Simple CNN (flattened for current implementation)
    sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_ADAM())
    
    fr fr Feature extraction layers
    sus conv_sim1 Layer = layer_create_dense(input_size, 512, ACTIVATION_RELU())
    sus pool_sim1 Layer = layer_create_dropout(0.25)
    sus conv_sim2 Layer = layer_create_dense(512, 256, ACTIVATION_RELU())
    sus pool_sim2 Layer = layer_create_dropout(0.25)
    
    fr fr Classification head
    sus fc1 Layer = layer_create_dense(256, 128, ACTIVATION_RELU())
    sus dropout Layer = layer_create_dropout(0.5)
    sus output Layer = layer_create_dense(128, num_classes, ACTIVATION_SOFTMAX())
    
    network = neural_network_add_layer(network, conv_sim1)
    network = neural_network_add_layer(network, pool_sim1)
    network = neural_network_add_layer(network, conv_sim2)
    network = neural_network_add_layer(network, pool_sim2)
    network = neural_network_add_layer(network, fc1)
    network = neural_network_add_layer(network, dropout)
    network = neural_network_add_layer(network, output)
    
    fr fr Training
    demo_neural_network_classification(train_data, train_labels, train_data, train_labels,
                                       num_samples, num_samples, input_size, num_classes, 50)
}
```

### 4. Hyperparameter Optimization

```cursed
slay demo_hyperparameter_search() cringe {
    sus input_size drip = 20
    sus num_classes drip = 3
    sus num_train drip = 400
    sus num_val drip = 100
    
    fr fr Generate datasets
    sus train_data []meal = generate_random_data(num_train, input_size)
    sus train_labels []meal = generate_random_labels(num_train, num_classes)
    sus val_data []meal = generate_random_data(num_val, input_size)
    sus val_labels []meal = generate_random_labels(num_val, num_classes)
    
    fr fr Grid search
    sus best_lr meal
    sus best_hidden drip
    sus best_acc meal
    (best_lr, best_hidden, best_acc) = grid_search_hyperparameters(
        train_data, train_labels, val_data, val_labels,
        num_train, num_val, input_size, num_classes)
    
    vibez.spill("Optimal hyperparameters found!")
    vibez.spill("Learning Rate: ", best_lr)
    vibez.spill("Hidden Size: ", best_hidden)
    vibez.spill("Best Accuracy: ", best_acc * 100.0, "%")
}
```

## 🚀 GPU Acceleration

### High-Performance Computing
The nnz module now includes comprehensive **GPU acceleration** for neural networks:

#### GPU Platform Support
- **CUDA**: Native NVIDIA GPU acceleration with cuBLAS/cuDNN integration  
- **OpenCL**: Cross-platform GPU support (AMD, Intel, NVIDIA)
- **Metal**: Apple Silicon GPU acceleration
- **Automatic Fallback**: Seamless CPU fallback when GPU unavailable

#### GPU Initialization
```cursed
fr fr Initialize GPU subsystem
sus gpu_available lit = gpu_initialize()

ready (gpu_available) {
    vibez.spill("GPU acceleration enabled!")
    sus device_info GPUDevice = gpu_get_device_info()
    vibez.spill("GPU: ", device_info.device_name)
    vibez.spill("Memory: ", device_info.memory_total / (1024*1024), " MB")
} otherwise {
    vibez.spill("Using CPU fallback")
}
```

#### GPU Device Management
```cursed
fr fr Get detailed GPU information
sus device_info GPUDevice = gpu_get_device_info()

ready (device_info.device_type == GPU_DEVICE_TYPE_CUDA()) {
    vibez.spill("CUDA Device - Compute ", device_info.compute_capability_major, 
                ".", device_info.compute_capability_minor)
}

fr fr Monitor GPU memory usage
sus allocated drip
sus total drip
(allocated, total) = gpu_get_memory_usage()
vibez.spill("GPU Memory: ", allocated/(1024*1024), "/", total/(1024*1024), " MB")
```

#### GPU-Accelerated Operations
```cursed
fr fr High-performance matrix multiplication
sus result []meal = gpu_matrix_multiply(matrix_a, matrix_b, m, n, k)

fr fr GPU-accelerated 2D convolution  
sus conv_result []meal = gpu_conv2d_forward(input, weights, biases,
                                           height, width, channels,
                                           kernel_size, out_channels,
                                           stride, padding)

fr fr Batch operations for maximum GPU utilization
gpu_batch_matrix_multiply(batch_inputs, weights, outputs, 
                         batch_size, input_size, output_size)
```

#### GPU Neural Network Training
```cursed
fr fr GPU-accelerated training epoch
sus loss meal = neural_network_train_epoch_gpu(network, train_data, train_labels,
                                              num_samples, input_size, batch_size)

fr fr GPU batch forward pass
sus predictions [][]meal = neural_network_forward_batch_gpu(network, batch_inputs)
```

#### Performance Benchmarks
Typical GPU acceleration speedups:
- **Matrix Multiplication**: 10-50x faster than CPU
- **Convolution Operations**: 15-100x faster than CPU  
- **Neural Network Training**: 5-20x faster than CPU
- **Batch Processing**: 10-30x faster than CPU

#### Memory Management
```cursed
fr fr Efficient GPU memory allocation
sus buffer GPUBuffer = gpu_allocate_buffer(size_bytes)

fr fr Copy data to/from GPU
gpu_copy_to_device(host_data, buffer)
gpu_copy_from_device(buffer, result_data)

fr fr Automatic cleanup
gpu_free_buffer(buffer)
gpu_cleanup()  fr fr Clean all GPU resources
```

### GPU Best Practices
1. **Batch Size**: Use larger batches (32-256) for better GPU utilization
2. **Memory Management**: Monitor GPU memory usage to avoid OOM errors  
3. **Data Transfer**: Minimize host-device transfers
4. **Mixed Precision**: Use appropriate data types for performance
5. **Async Operations**: Overlap computation and data transfer when possible

### Performance Optimization

### Memory Optimization
- Efficient tensor operations through `tensorz` module
- In-place operations where possible
- Batch processing for memory efficiency

### Training Optimization
- Mini-batch gradient descent
- Efficient forward/backward propagation
- Optimized activation function implementations

## Integration

The nnz module integrates seamlessly with:
- **mlz**: Machine learning primitives and algorithms
- **mathz**: Mathematical operations and functions
- **tensorz**: Tensor operations and linear algebra
- **arrayz**: Array manipulation utilities
- **vibez**: I/O operations for progress tracking

## Best Practices

1. **Data Preprocessing**: Always normalize input data
2. **Learning Rate**: Start with default values, tune as needed
3. **Regularization**: Use dropout and weight decay to prevent overfitting
4. **Early Stopping**: Monitor validation loss to avoid overtraining
5. **Batch Size**: Choose based on memory constraints and training stability
6. **Architecture**: Start simple, add complexity as needed

## Examples in Action

Run the built-in demonstrations:
```cursed
demo_neural_network_classification(...)  fr fr Multi-class classification
demo_deep_autoencoder(...)               fr fr Unsupervised learning
demo_transfer_learning(...)              fr fr Fine-tuning pretrained models
demo_hyperparameter_search(...)          fr fr Automated hyperparameter tuning
```

The nnz module provides a solid foundation for deep learning in CURSED, enabling everything from simple neural networks to complex architectures with state-of-the-art training techniques.
