# MLz - CURSED Machine Learning Module

A comprehensive machine learning library implemented in pure CURSED, providing fundamental ML algorithms, neural networks, and data processing capabilities.

## Features

### Activation Functions
- **Sigmoid**: `sigmoid()`, `sigmoid_derivative()`
- **Hyperbolic Tangent**: `tanh_activation()`, `tanh_derivative()`
- **ReLU**: `relu()`, `relu_derivative()`
- **Leaky ReLU**: `leaky_relu()`, `leaky_relu_derivative()`
- **Linear**: `linear_activation()`, `linear_derivative()`
- **Tensor application**: `apply_activation()`, `apply_activation_derivative()`

### Loss Functions
- **Regression**: `mean_squared_error()`, `mean_absolute_error()`
- **Classification**: `binary_cross_entropy()`
- **Numerical stability**: Built-in epsilon handling for log operations

### Linear Models
- **Linear Regression**: Complete implementation with gradient descent
- **Model creation**: `linear_model_create()`
- **Prediction**: `linear_model_predict()`
- **Training**: `linear_model_train_step()` with regularization support

### Neural Networks
- **Layer creation**: `layer_create()` with configurable activation functions
- **Forward propagation**: `layer_forward()`, `neural_network_forward()`
- **Network creation**: `neural_network_create()` with flexible architecture
- **Training**: `neural_network_train_batch()` with backpropagation

### Optimization Algorithms
- **Stochastic Gradient Descent**: `sgd_update()`
- **Momentum**: `momentum_update()`
- **Adam Optimizer**: `adam_update()` with adaptive learning rates
- **State management**: `GradientDescentState` for optimizer state tracking

### Clustering
- **K-Means**: Complete implementation with initialization, assignment, and updating
- **Centroid initialization**: `kmeans_init_centroids()`
- **Cluster assignment**: `kmeans_assign_clusters()`
- **Full algorithm**: `kmeans_cluster()` with convergence detection

### Evaluation Metrics
- **Classification**: `accuracy_score()` with configurable threshold
- **Regression**: `r2_score()` (coefficient of determination)
- **Model validation**: Built-in performance measurement

### Data Preprocessing
- **Train-test split**: `train_test_split()` with configurable ratio
- **Feature standardization**: `standardize_features()` (z-score normalization)
- **Dataset utilities**: `create_dataset()`, `dataset_shuffle()`

### Training Management
- **Training history**: `TrainingHistory` struct and utilities
- **Progress tracking**: `print_training_progress()`
- **History updates**: `update_training_history()`

## Data Structures

### Core ML Structures
```cursed
squad LinearModel {
    spill weights Tensor
    spill bias meal
    spill learning_rate meal
    spill regularization meal
}

squad NeuralNetwork {
    spill layers []Layer
    spill num_layers normie
    spill learning_rate meal
    spill loss_function tea
}

squad Layer {
    spill weights Tensor
    spill biases Tensor
    spill activation tea
    spill input_size normie
    spill output_size normie
}
```

### Training and Evaluation
```cursed
squad Dataset {
    spill features Tensor
    spill labels Tensor
    spill num_samples normie
    spill num_features normie
}

squad TrainingHistory {
    spill losses []meal
    spill accuracies []meal
    spill epochs normie
}

squad GradientDescentState {
    spill weights Tensor
    spill bias meal
    spill momentum_weights Tensor
    spill momentum_bias meal
    spill velocity_weights Tensor
    spill velocity_bias meal
}
```

## Usage Examples

### Linear Regression
```cursed
yeet "mlz"
yeet "tensorz"

fr fr Create training data
sus features_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]  fr fr [[1,2], [3,4], [5,6]]
sus labels_data []meal = [3.0, 7.0, 11.0]  fr fr y = x1 + x2
sus feature_shape []normie = [3, 2]
sus label_shape []normie = [3]
sus features Tensor = tensor_from_array(features_data, feature_shape)
sus labels Tensor = tensor_from_array(labels_data, label_shape)

fr fr Create and train model
sus model LinearModel = linear_model_create(2, 0.01)
sus epoch normie = 0
bestie epoch < 1000 {
    model = linear_model_train_step(model, features, labels)
    epoch = epoch + 1
}

fr fr Make predictions
sus predictions Tensor = linear_model_predict(model, features)
sus mse meal = mean_squared_error(predictions, labels)
vibez.spill("Final MSE: ", mse)
```

### Neural Network
```cursed
fr fr Define network architecture
sus layer_sizes []normie = [2, 4, 1]  fr fr 2 inputs, 4 hidden, 1 output
sus activations []tea = ["relu", "sigmoid"]
sus network NeuralNetwork = neural_network_create(layer_sizes, activations, 0.01)

fr fr Train the network
sus training_features Tensor = features  fr fr From previous example
sus training_labels Tensor = labels
sus batch_epoch normie = 0
bestie batch_epoch < 100 {
    network = neural_network_train_batch(network, training_features, training_labels)
    batch_epoch = batch_epoch + 1
}

fr fr Get predictions
sus nn_predictions Tensor = neural_network_forward(network, training_features)
```

### K-Means Clustering
```cursed
fr fr Create sample data (2 clusters)
sus cluster_data []meal = [1.0, 1.0, 1.1, 1.1, 5.0, 5.0, 5.1, 5.1]
sus cluster_shape []normie = [4, 2]
sus cluster_features Tensor = tensor_from_array(cluster_data, cluster_shape)

fr fr Perform clustering
sus centroids Tensor
sus assignments Tensor
(centroids, assignments) = kmeans_cluster(cluster_features, 2, 100)

fr fr Print results
vibez.spill("Cluster assignments:")
sus i normie = 0
bestie i < assignments.size {
    vibez.spill("Point ", i, ": Cluster ", assignments.data[i])
    i = i + 1
}
```

### Data Preprocessing
```cursed
fr fr Train-test split
sus train_x Tensor
sus test_x Tensor
sus train_y Tensor
sus test_y Tensor
(train_x, test_x, train_y, test_y) = train_test_split(features, labels, 0.2)

fr fr Feature standardization
sus standardized_features Tensor
sus feature_means Tensor
sus feature_stds Tensor
(standardized_features, feature_means, feature_stds) = standardize_features(train_x)
```

### Activation Functions
```cursed
fr fr Apply activation functions to scalars
sus sigmoid_result meal = sigmoid(0.5)
sus relu_result meal = relu(-2.0)
sus tanh_result meal = tanh_activation(1.0)

fr fr Apply to tensors
sus tensor_data []meal = [-1.0, 0.0, 1.0, 2.0]
sus tensor_shape []normie = [4]
sus input_tensor Tensor = tensor_from_array(tensor_data, tensor_shape)

sus relu_tensor Tensor = apply_activation(input_tensor, "relu")
sus sigmoid_tensor Tensor = apply_activation(input_tensor, "sigmoid")
```

### Training with History Tracking
```cursed
fr fr Initialize training history
sus history TrainingHistory = create_training_history()

fr fr Training loop with progress tracking
sus model LinearModel = linear_model_create(2, 0.01)
sus epoch normie = 0
bestie epoch < 100 {
    model = linear_model_train_step(model, features, labels)
    
    lowkey epoch % 10 == 0 {
        sus predictions Tensor = linear_model_predict(model, features)
        sus loss meal = mean_squared_error(predictions, labels)
        sus accuracy meal = r2_score(predictions, labels)
        
        history = update_training_history(history, loss, accuracy)
        print_training_progress(epoch, loss, accuracy)
    }
    epoch = epoch + 1
}
```

### Evaluation Metrics
```cursed
fr fr Binary classification accuracy
sus pred_data []meal = [0.8, 0.3, 0.9, 0.1]
sus target_data []meal = [1.0, 0.0, 1.0, 0.0]
sus pred_shape []normie = [4]
sus predictions Tensor = tensor_from_array(pred_data, pred_shape)
sus targets Tensor = tensor_from_array(target_data, pred_shape)

sus accuracy meal = accuracy_score(predictions, targets, 0.5)
vibez.spill("Accuracy: ", accuracy * 100.0, "%")

fr fr Regression R² score
sus r2 meal = r2_score(predictions, targets)
vibez.spill("R² Score: ", r2)
```

## Advanced Features

### Custom Optimizers
```cursed
fr fr SGD with momentum
sus gd_state GradientDescentState = gradient_descent_state_create(num_features)
sus gradients Tensor = compute_gradients()  fr fr Your gradient computation
gd_state = momentum_update(gd_state, gradients, 0.05, 0.01, 0.9)

fr fr Adam optimizer
gd_state = adam_update(gd_state, gradients, 0.001, 0.9, 0.999, 1e-8)
```

### Dataset Management
```cursed
fr fr Create and shuffle dataset
sus dataset Dataset = create_dataset(features, labels)
sus shuffled_dataset Dataset = dataset_shuffle(dataset)

fr fr Access dataset properties
vibez.spill("Samples: ", dataset.num_samples)
vibez.spill("Features: ", dataset.num_features)
```

## Performance Considerations

- **Vectorized operations**: Uses tensor operations for efficiency
- **Numerical stability**: Includes epsilon handling and safe divisions
- **Memory efficiency**: Efficient tensor storage and reuse
- **Convergence detection**: Built-in convergence checking for iterative algorithms

## Error Handling

- **Graceful degradation**: Invalid inputs return safe default values
- **Bounds checking**: Array and tensor access is bounds-checked
- **Numerical safety**: Division by zero and log of zero are handled safely
- **Shape validation**: Tensor operations validate compatible shapes

## Dependencies

- `tensorz`: Tensor operations and linear algebra
- `mathz`: Mathematical functions and constants
- `arrayz`: Array manipulation utilities

## Testing

Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/mlz/test_mlz.💀
```

The test suite covers:
- All activation functions and their derivatives
- Loss functions with various input scenarios
- Linear regression training and prediction
- Neural network forward propagation
- K-means clustering algorithm
- Optimization algorithms (SGD, momentum, Adam)
- Data preprocessing functions
- Evaluation metrics
- Edge cases and error conditions

## Integration Examples

### Complete ML Pipeline
```cursed
yeet "mlz"
yeet "tensorz"

fr fr 1. Load and preprocess data
sus raw_features Tensor = load_data()  fr fr Your data loading
sus standardized_features Tensor
sus means Tensor
sus stds Tensor
(standardized_features, means, stds) = standardize_features(raw_features)

fr fr 2. Split data
sus train_x Tensor
sus test_x Tensor
sus train_y Tensor  
sus test_y Tensor
(train_x, test_x, train_y, test_y) = train_test_split(standardized_features, labels, 0.2)

fr fr 3. Train model
sus model LinearModel = linear_model_create(num_features, 0.01)
sus history TrainingHistory = create_training_history()

sus epoch normie = 0
bestie epoch < 1000 {
    model = linear_model_train_step(model, train_x, train_y)
    
    lowkey epoch % 100 == 0 {
        sus train_pred Tensor = linear_model_predict(model, train_x)
        sus train_loss meal = mean_squared_error(train_pred, train_y)
        sus train_r2 meal = r2_score(train_pred, train_y)
        
        history = update_training_history(history, train_loss, train_r2)
        print_training_progress(epoch, train_loss, train_r2)
    }
    epoch = epoch + 1
}

fr fr 4. Evaluate on test set
sus test_predictions Tensor = linear_model_predict(model, test_x)
sus test_mse meal = mean_squared_error(test_predictions, test_y)
sus test_r2 meal = r2_score(test_predictions, test_y)

vibez.spill("Final Test MSE: ", test_mse)
vibez.spill("Final Test R²: ", test_r2)
```

## Implementation Notes

- **Pure CURSED**: Completely implemented in CURSED without FFI dependencies
- **Modular design**: Each algorithm is independently usable
- **Extensible architecture**: Easy to add new algorithms and optimizers
- **Educational focus**: Clear, readable implementations suitable for learning
- **Production ready**: Includes proper error handling and numerical stability
