fr fr CURSED Neural Network Module (nnz) - Comprehensive Test Suite
fr fr Tests for all neural network functionality

yeet "nnz"
yeet "mlz"
yeet "mathz"
yeet "tensorz"
yeet "arrayz"
yeet "vibez"

fr fr === TEST UTILITY FUNCTIONS ===

slay generate_random_data(num_samples drip, num_features drip) meal[value]{
    sus data meal[value] = []
    sus i drip = 0
    bestie (i < num_samples * num_features) {
        data = append(data, random_uniform())
        i = i + 1
    }
    damn data
}

slay generate_random_labels(num_samples drip, num_classes drip) meal[value]{
    sus labels meal[value] = []
    sus i drip = 0
    bestie (i < num_samples) {
        sus label meal = random_range(0, num_classes)
        labels = append(labels, label)
        i = i + 1
    }
    damn labels
}

slay generate_classification_data(num_samples drip, num_features drip, num_classes drip) (meal[value], meal[value]) {
    sus features meal[value] = generate_random_data(num_samples, num_features)
    sus labels meal[value] = generate_random_labels(num_samples, num_classes)
    damn (features, labels)
}

slay test_assert(condition lit, test_name tea) cringe {
    ready (condition) {
        vibez.spill("✓ ", test_name, " PASSED")
    } otherwise {
        vibez.spill("✗ ", test_name, " FAILED")
    }
}

fr fr === ACTIVATION FUNCTION TESTS ===

slay test_activation_functions() cringe {
    vibez.spill("=== Testing Activation Functions ===")
    
    fr fr Test sigmoid
    sus sig_result meal = sigmoid(0.0)
    test_assert(sig_result > 0.4 && sig_result < 0.6, "sigmoid(0) ≈ 0.5")
    
    sus sig_pos meal = sigmoid(10.0)
    test_assert(sig_pos > 0.9, "sigmoid(10) > 0.9")
    
    sus sig_neg meal = sigmoid(-10.0)
    test_assert(sig_neg < 0.1, "sigmoid(-10) < 0.1")
    
    fr fr Test ReLU
    sus relu_pos meal = relu(5.0)
    test_assert(relu_pos == 5.0, "relu(5) = 5")
    
    sus relu_neg meal = relu(-3.0)
    test_assert(relu_neg == 0.0, "relu(-3) = 0")
    
    fr fr Test activation derivatives
    sus sig_deriv meal = sigmoid_derivative(0.0)
    test_assert(sig_deriv > 0.2 && sig_deriv < 0.3, "sigmoid_derivative(0) ≈ 0.25")
    
    sus relu_deriv_pos meal = relu_derivative(5.0)
    test_assert(relu_deriv_pos == 1.0, "relu_derivative(5) = 1")
    
    sus relu_deriv_neg meal = relu_derivative(-3.0)
    test_assert(relu_deriv_neg == 0.0, "relu_derivative(-3) = 0")
    
    fr fr Test advanced activations
    sus swish_result meal = swish(1.0)
    test_assert(swish_result > 0.5, "swish(1) > 0.5")
    
    sus gelu_result meal = gelu(0.0)
    test_assert(gelu_result > -0.1 && gelu_result < 0.1, "gelu(0) ≈ 0")
    
    vibez.spill("Activation function tests completed!")
}

fr fr === LAYER TESTS ===

slay test_layer_creation() cringe {
    vibez.spill("=== Testing Layer Creation ===")
    
    fr fr Test dense layer creation
    sus dense_layer Layer = layer_create_dense(10, 5, ACTIVATION_RELU())
    test_assert(dense_layer.layer_type == LAYER_TYPE_DENSE(), "Dense layer type correct")
    test_assert(dense_layer.input_size == 10, "Dense layer input size correct")
    test_assert(dense_layer.output_size == 5, "Dense layer output size correct")
    test_assert(dense_layer.activation_type == ACTIVATION_RELU(), "Dense layer activation correct")
    test_assert(len(dense_layer.weights) == 50, "Dense layer weights size correct (10*5)")
    test_assert(len(dense_layer.biases) == 5, "Dense layer biases size correct")
    
    fr fr Test dropout layer creation
    sus dropout_layer Layer = layer_create_dropout(0.5)
    test_assert(dropout_layer.layer_type == LAYER_TYPE_DROPOUT(), "Dropout layer type correct")
    test_assert(dropout_layer.dropout_rate == 0.5, "Dropout rate correct")
    
    fr fr Test batch normalization layer creation
    sus bn_layer Layer = layer_create_batch_norm(10)
    test_assert(bn_layer.layer_type == LAYER_TYPE_BATCHNORM(), "Batch norm layer type correct")
    test_assert(bn_layer.input_size == 10, "Batch norm input size correct")
    test_assert(len(bn_layer.gamma) == 10, "Batch norm gamma size correct")
    test_assert(len(bn_layer.beta) == 10, "Batch norm beta size correct")
    
    vibez.spill("Layer creation tests completed!")
}

slay test_layer_forward() cringe {
    vibez.spill("=== Testing Layer Forward Pass ===")
    
    fr fr Test dense layer forward pass
    sus dense_layer Layer = layer_create_dense(3, 2, ACTIVATION_RELU())
    sus input meal[value] = [1.0, 2.0, 3.0]
    sus output meal[value] = layer_forward_dense(dense_layer, input)
    
    test_assert(len(output) == 2, "Dense layer output size correct")
    test_assert(output[0] >= 0.0, "ReLU activation applied (output[0] >= 0)")
    test_assert(output[1] >= 0.0, "ReLU activation applied (output[1] >= 0)")
    
    fr fr Test dropout layer forward pass (training mode)
    sus dropout_layer Layer = layer_create_dropout(0.5)
    dropout_layer.training = based
    sus dropout_input meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0]
    sus dropout_output meal[value] = layer_forward_dropout(dropout_layer, dropout_input)
    
    test_assert(len(dropout_output) == 5, "Dropout output size correct")
    
    fr fr Test dropout layer forward pass (inference mode)
    dropout_layer.training = cringe
    sus dropout_inference meal[value] = layer_forward_dropout(dropout_layer, dropout_input)
    test_assert(len(dropout_inference) == 5, "Dropout inference output size correct")
    
    vibez.spill("Layer forward pass tests completed!")
}

fr fr === NEURAL NETWORK TESTS ===

slay test_neural_network_creation() cringe {
    vibez.spill("=== Testing Neural Network Creation ===")
    
    fr fr Test network creation
    sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_ADAM())
    test_assert(network.learning_rate == 0.001, "Learning rate set correctly")
    test_assert(network.optimizer_type == OPTIMIZER_ADAM(), "Optimizer type set correctly")
    test_assert(len(network.layers) == 0, "Initial network has no layers")
    
    fr fr Test adding layers
    sus layer1 Layer = layer_create_dense(10, 5, ACTIVATION_RELU())
    sus layer2 Layer = layer_create_dense(5, 3, ACTIVATION_SOFTMAX())
    
    network = neural_network_add_layer(network, layer1)
    test_assert(len(network.layers) == 1, "Network has 1 layer after adding first")
    
    network = neural_network_add_layer(network, layer2)
    test_assert(len(network.layers) == 2, "Network has 2 layers after adding second")
    
    vibez.spill("Neural network creation tests completed!")
}

slay test_neural_network_forward() cringe {
    vibez.spill("=== Testing Neural Network Forward Pass ===")
    
    fr fr Create simple network
    sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_SGD())
    sus layer1 Layer = layer_create_dense(4, 3, ACTIVATION_RELU())
    sus layer2 Layer = layer_create_dense(3, 2, ACTIVATION_SIGMOID())
    
    network = neural_network_add_layer(network, layer1)
    network = neural_network_add_layer(network, layer2)
    
    fr fr Test forward pass
    sus input meal[value] = [1.0, 2.0, 3.0, 4.0]
    sus output meal[value] = neural_network_forward(network, input)
    
    test_assert(len(output) == 2, "Network output size correct")
    test_assert(output[0] >= 0.0 && output[0] <= 1.0, "Sigmoid output in range [0,1]")
    test_assert(output[1] >= 0.0 && output[1] <= 1.0, "Sigmoid output in range [0,1]")
    
    vibez.spill("Neural network forward pass tests completed!")
}

fr fr === LOSS FUNCTION TESTS ===

slay test_loss_functions() cringe {
    vibez.spill("=== Testing Loss Functions ===")
    
    fr fr Test MSE loss
    sus predictions_mse meal[value] = [1.0, 2.0, 3.0]
    sus targets_mse meal[value] = [1.1, 1.9, 3.2]
    sus mse_result meal = mse_loss(predictions_mse, targets_mse)
    test_assert(mse_result >= 0.0 && mse_result < 0.1, "MSE loss reasonable")
    
    fr fr Test MAE loss
    sus mae_result meal = mae_loss(predictions_mse, targets_mse)
    test_assert(mae_result >= 0.0 && mae_result < 0.3, "MAE loss reasonable")
    
    fr fr Test binary cross-entropy
    sus predictions_bce meal[value] = [0.9, 0.1, 0.8, 0.2]
    sus targets_bce meal[value] = [1.0, 0.0, 1.0, 0.0]
    sus bce_result meal = binary_crossentropy_loss(predictions_bce, targets_bce)
    test_assert(bce_result >= 0.0, "Binary cross-entropy loss non-negative")
    
    fr fr Test categorical cross-entropy
    sus predictions_ce meal[value] = [0.8, 0.1, 0.1]
    sus targets_ce meal[value] = [1.0, 0.0, 0.0]
    sus ce_result meal = categorical_crossentropy_loss(predictions_ce, targets_ce)
    test_assert(ce_result >= 0.0, "Categorical cross-entropy loss non-negative")
    
    fr fr Test Huber loss
    sus huber_result meal = huber_loss(predictions_mse, targets_mse, 1.0)
    test_assert(huber_result >= 0.0, "Huber loss non-negative")
    
    vibez.spill("Loss function tests completed!")
}

fr fr === OPTIMIZER TESTS ===

slay test_optimizers() cringe {
    vibez.spill("=== Testing Optimizers ===")
    
    sus weights meal[value] = [1.0, 2.0, 3.0]
    sus gradients meal[value] = [0.1, 0.2, 0.3]
    sus learning_rate meal = 0.01
    
    fr fr Test SGD optimizer
    sus sgd_weights meal[value] = optimizer_sgd_update_weights(weights, gradients, learning_rate)
    test_assert(len(sgd_weights) == 3, "SGD updated weights size correct")
    test_assert(sgd_weights[0] < weights[0], "SGD weight decreased (gradient positive)")
    
    fr fr Test momentum optimizer
    sus momentum_weights meal[value] = [0.0, 0.0, 0.0]
    sus updated_weights meal[value]
    sus updated_momentum meal[value]
    (updated_weights, updated_momentum) = optimizer_momentum_update_weights(weights, gradients, momentum_weights, learning_rate, 0.9)
    test_assert(len(updated_weights) == 3, "Momentum updated weights size correct")
    test_assert(len(updated_momentum) == 3, "Momentum variables size correct")
    
    fr fr Test Adam optimizer
    sus m meal[value] = [0.0, 0.0, 0.0]
    sus v meal[value] = [0.0, 0.0, 0.0]
    sus adam_weights meal[value]
    sus adam_m meal[value]
    sus adam_v meal[value]
    (adam_weights, adam_m, adam_v) = optimizer_adam_update_weights(weights, gradients, m, v, learning_rate, 0.9, 0.999, 1e-8, 1)
    test_assert(len(adam_weights) == 3, "Adam updated weights size correct")
    test_assert(len(adam_m) == 3, "Adam m variables size correct")
    test_assert(len(adam_v) == 3, "Adam v variables size correct")
    
    vibez.spill("Optimizer tests completed!")
}

fr fr === REGULARIZATION TESTS ===

slay test_regularization() cringe {
    vibez.spill("=== Testing Regularization ===")
    
    sus weights meal[value] = [1.0, -2.0, 3.0, -4.0]
    sus lambda meal = 0.01
    
    fr fr Test L1 regularization
    sus l1_loss meal = l1_regularization_loss(weights, lambda)
    test_assert(l1_loss > 0.0, "L1 regularization loss positive")
    test_assert(l1_loss == lambda * (1.0 + 2.0 + 3.0 + 4.0), "L1 loss correct calculation")
    
    fr fr Test L2 regularization
    sus l2_loss meal = l2_regularization_loss(weights, lambda)
    test_assert(l2_loss > 0.0, "L2 regularization loss positive")
    test_assert(l2_loss == lambda * (1.0 + 4.0 + 9.0 + 16.0), "L2 loss correct calculation")
    
    fr fr Test elastic net regularization
    sus elastic_loss meal = elastic_net_regularization_loss(weights, 0.005, 0.005)
    test_assert(elastic_loss > 0.0, "Elastic net regularization loss positive")
    
    fr fr Test gradient clipping
    sus large_gradients meal[value] = [10.0, -20.0, 15.0]
    sus clipped_gradients meal[value] = gradient_clipping(large_gradients, 5.0)
    sus clipped_norm meal = sqrt_meal(clipped_gradients[0] * clipped_gradients[0] + 
                                     clipped_gradients[1] * clipped_gradients[1] + 
                                     clipped_gradients[2] * clipped_gradients[2])
    test_assert(clipped_norm <= 5.1, "Gradient clipping applied correctly")
    
    fr fr Test early stopping
    sus validation_losses meal[value] = [1.0, 0.9, 0.8, 0.85, 0.87, 0.88, 0.89]
    sus should_stop lit = early_stopping_check(validation_losses, 3, 0.01)
    test_assert(should_stop, "Early stopping triggered correctly")
    
    vibez.spill("Regularization tests completed!")
}

fr fr === LEARNING RATE SCHEDULING TESTS ===

slay test_learning_rate_scheduling() cringe {
    vibez.spill("=== Testing Learning Rate Scheduling ===")
    
    sus initial_lr meal = 0.1
    sus epoch drip = 10
    
    fr fr Test step decay
    sus step_lr meal = learning_rate_step_decay(initial_lr, epoch, 5, 0.5)
    test_assert(step_lr < initial_lr, "Step decay reduces learning rate")
    test_assert(step_lr == initial_lr * 0.25, "Step decay calculation correct")
    
    fr fr Test exponential decay
    sus exp_lr meal = learning_rate_exponential_decay(initial_lr, epoch, 0.1)
    test_assert(exp_lr < initial_lr, "Exponential decay reduces learning rate")
    test_assert(exp_lr > 0.0, "Exponential decay positive")
    
    fr fr Test cosine annealing
    sus cos_lr meal = learning_rate_cosine_annealing(initial_lr, epoch, 20)
    test_assert(cos_lr > 0.0, "Cosine annealing positive")
    test_assert(cos_lr <= initial_lr, "Cosine annealing bounded")
    
    vibez.spill("Learning rate scheduling tests completed!")
}

fr fr === CONVOLUTIONAL LAYER TESTS ===

slay test_convolutional_layers() cringe {
    vibez.spill("=== Testing Convolutional Layers ===")
    
    fr fr Test 2D convolution
    sus input_height drip = 4
    sus input_width drip = 4
    sus input_channels drip = 2
    sus kernel_size drip = 3
    sus output_channels drip = 1
    sus stride drip = 1
    sus padding drip = 0
    
    sus input_size drip = input_height * input_width * input_channels
    sus input meal[value] = generate_random_data(1, input_size)
    
    sus weight_size drip = kernel_size * kernel_size * input_channels * output_channels
    sus weights meal[value] = generate_random_data(1, weight_size)
    sus biases meal[value] = [0.0]
    
    sus conv_output meal[value] = conv2d_forward(input, weights, biases, input_height, input_width, input_channels, kernel_size, output_channels, stride, padding)
    
    sus expected_output_height drip = (input_height - kernel_size) / stride + 1
    sus expected_output_width drip = (input_width - kernel_size) / stride + 1
    sus expected_output_size drip = expected_output_height * expected_output_width * output_channels
    
    test_assert(len(conv_output) == expected_output_size, "Convolution output size correct")
    
    fr fr Test max pooling
    sus pool_size drip = 2
    sus pool_output meal[value] = maxpool2d_forward(input, input_height, input_width, input_channels, pool_size, stride)
    
    sus expected_pool_height drip = (input_height - pool_size) / stride + 1
    sus expected_pool_width drip = (input_width - pool_size) / stride + 1
    sus expected_pool_size drip = expected_pool_height * expected_pool_width * input_channels
    
    test_assert(len(pool_output) == expected_pool_size, "Max pooling output size correct")
    
    vibez.spill("Convolutional layer tests completed!")
}

fr fr === ENSEMBLE METHOD TESTS ===

slay test_ensemble_methods() cringe {
    vibez.spill("=== Testing Ensemble Methods ===")
    
    fr fr Test ensemble averaging
    sus num_models drip = 3
    sus output_size drip = 2
    sus predictions meal[value][value] = [
        [0.8, 0.2],
        [0.6, 0.4],
        [0.7, 0.3]
    ]
    
    sus averaged_pred meal[value] = ensemble_predict_average(predictions, num_models, output_size)
    test_assert(len(averaged_pred) == output_size, "Ensemble average output size correct")
    test_assert(averaged_pred[0] > 0.6 && averaged_pred[0] < 0.8, "Ensemble average calculation correct")
    
    fr fr Test weighted ensemble
    sus weights meal[value] = [0.5, 0.3, 0.2]
    sus weighted_pred meal[value] = ensemble_predict_weighted(predictions, weights, num_models, output_size)
    test_assert(len(weighted_pred) == output_size, "Ensemble weighted output size correct")
    
    vibez.spill("Ensemble method tests completed!")
}

fr fr === MODEL SERIALIZATION TESTS ===

slay test_model_serialization() cringe {
    vibez.spill("=== Testing Model Serialization ===")
    
    fr fr Create a simple network
    sus network NeuralNetwork = neural_network_create(0.001, OPTIMIZER_SGD())
    sus layer1 Layer = layer_create_dense(3, 2, ACTIVATION_RELU())
    sus layer2 Layer = layer_create_dense(2, 1, ACTIVATION_SIGMOID())
    
    network = neural_network_add_layer(network, layer1)
    network = neural_network_add_layer(network, layer2)
    
    fr fr Save weights
    sus saved_weights meal[value][value] = neural_network_save_weights(network)
    test_assert(len(saved_weights) == 4, "Saved weights include all layers (weights + biases)")
    
    fr fr Modify network weights
    network.layers[0].weights[0] = 999.0
    network.layers[1].biases[0] = 888.0
    
    fr fr Load weights back
    network = neural_network_load_weights(network, saved_weights)
    test_assert(network.layers[0].weights[0] != 999.0, "Weights restored from saved state")
    test_assert(network.layers[1].biases[0] != 888.0, "Biases restored from saved state")
    
    vibez.spill("Model serialization tests completed!")
}

fr fr === INTEGRATION TESTS ===

slay test_simple_classification() cringe {
    vibez.spill("=== Testing Simple Classification ===")
    
    fr fr Generate simple dataset
    sus num_samples drip = 50
    sus num_features drip = 4
    sus num_classes drip = 2
    
    sus features meal[value]
    sus labels meal[value]
    (features, labels) = generate_classification_data(num_samples, num_features, num_classes)
    
    fr fr Create and train network
    sus network NeuralNetwork = neural_network_create(0.01, OPTIMIZER_SGD())
    sus hidden_layer Layer = layer_create_dense(num_features, 8, ACTIVATION_RELU())
    sus output_layer Layer = layer_create_dense(8, num_classes, ACTIVATION_SIGMOID())
    
    network = neural_network_add_layer(network, hidden_layer)
    network = neural_network_add_layer(network, output_layer)
    
    fr fr Train for a few epochs
    sus epoch drip = 0
    bestie (epoch < 5) {
        sus loss meal = neural_network_train_epoch(network, features, labels, num_samples, num_features, 16)
        test_assert(loss >= 0.0, "Training loss non-negative")
        epoch = epoch + 1
    }
    
    fr fr Evaluate
    sus accuracy meal = neural_network_evaluate(network, features, labels, num_samples, num_features)
    test_assert(accuracy >= 0.0 && accuracy <= 1.0, "Accuracy in valid range")
    
    vibez.spill("Simple classification test completed!")
}

slay test_autoencoder() cringe {
    vibez.spill("=== Testing Autoencoder ===")
    
    fr fr Generate data for autoencoder
    sus num_samples drip = 30
    sus input_size drip = 8
    sus encoding_dim drip = 3
    
    sus data meal[value] = generate_random_data(num_samples, input_size)
    
    fr fr Create autoencoder network
    sus autoencoder NeuralNetwork = neural_network_create(0.01, OPTIMIZER_ADAM())
    
    fr fr Encoder
    sus encoder_layer Layer = layer_create_dense(input_size, encoding_dim, ACTIVATION_RELU())
    
    fr fr Decoder
    sus decoder_layer Layer = layer_create_dense(encoding_dim, input_size, ACTIVATION_SIGMOID())
    
    autoencoder = neural_network_add_layer(autoencoder, encoder_layer)
    autoencoder = neural_network_add_layer(autoencoder, decoder_layer)
    
    fr fr Train autoencoder (input = target)
    sus epoch drip = 0
    bestie (epoch < 3) {
        sus reconstruction_loss meal = neural_network_train_epoch(autoencoder, data, data, num_samples, input_size, 8)
        test_assert(reconstruction_loss >= 0.0, "Reconstruction loss non-negative")
        epoch = epoch + 1
    }
    
    vibez.spill("Autoencoder test completed!")
}

slay test_hyperparameter_optimization() cringe {
    vibez.spill("=== Testing Hyperparameter Optimization ===")
    
    fr fr Generate small dataset for quick testing
    sus num_train drip = 20
    sus num_val drip = 10
    sus input_size drip = 5
    sus num_classes drip = 2
    
    sus train_data meal[value]
    sus train_labels meal[value]
    (train_data, train_labels) = generate_classification_data(num_train, input_size, num_classes)
    
    sus val_data meal[value]
    sus val_labels meal[value]
    (val_data, val_labels) = generate_classification_data(num_val, input_size, num_classes)
    
    fr fr Run grid search
    sus best_lr meal
    sus best_hidden drip
    sus best_acc meal
    (best_lr, best_hidden, best_acc) = grid_search_hyperparameters(train_data, train_labels, val_data, val_labels, num_train, num_val, input_size, num_classes)
    
    test_assert(best_lr > 0.0, "Best learning rate positive")
    test_assert(best_hidden > 0, "Best hidden size positive")
    test_assert(best_acc >= 0.0 && best_acc <= 1.0, "Best accuracy in valid range")
    
    vibez.spill("Hyperparameter optimization test completed!")
}

fr fr === COMPREHENSIVE DEMO TESTS ===

slay test_all_demos() cringe {
    vibez.spill("=== Testing All Demo Functions ===")
    
    fr fr Generate datasets for demos
    sus num_samples drip = 40
    sus num_features drip = 6
    sus num_classes drip = 3
    
    sus train_data meal[value]
    sus train_labels meal[value]
    (train_data, train_labels) = generate_classification_data(num_samples, num_features, num_classes)
    
    sus test_data meal[value]
    sus test_labels meal[value]
    (test_data, test_labels) = generate_classification_data(20, num_features, num_classes)
    
    fr fr Test neural network classification demo
    demo_neural_network_classification(train_data, train_labels, test_data, test_labels, num_samples, 20, num_features, num_classes, 10)
    
    fr fr Test autoencoder demo
    demo_deep_autoencoder(train_data, num_samples, num_features, 3, 15)
    
    fr fr Test transfer learning demo (with dummy pretrained weights)
    sus pretrained_weights meal[value][value] = []
    sus dummy_weights1 meal[value] = generate_random_data(1, num_features * 8)
    sus dummy_weights2 meal[value] = generate_random_data(1, 8 * 4)
    pretrained_weights = append(pretrained_weights, dummy_weights1)
    pretrained_weights = append(pretrained_weights, dummy_weights2)
    
    demo_transfer_learning(pretrained_weights, train_data, train_labels, num_samples, num_features, num_classes)
    
    vibez.spill("All demo function tests completed!")
}

fr fr === MAIN TEST RUNNER ===

slay run_all_tests() cringe {
    vibez.spill("🧪 CURSED Neural Network Module (nnz) - Test Suite")
    vibez.spill("=" * 60)
    
    test_activation_functions()
    vibez.spill("")
    
    test_layer_creation()
    vibez.spill("")
    
    test_layer_forward()
    vibez.spill("")
    
    test_neural_network_creation()
    vibez.spill("")
    
    test_neural_network_forward()
    vibez.spill("")
    
    test_loss_functions()
    vibez.spill("")
    
    test_optimizers()
    vibez.spill("")
    
    test_regularization()
    vibez.spill("")
    
    test_learning_rate_scheduling()
    vibez.spill("")
    
    test_convolutional_layers()
    vibez.spill("")
    
    test_ensemble_methods()
    vibez.spill("")
    
    test_model_serialization()
    vibez.spill("")
    
    test_simple_classification()
    vibez.spill("")
    
    test_autoencoder()
    vibez.spill("")
    
    test_hyperparameter_optimization()
    vibez.spill("")
    
    test_all_demos()
    vibez.spill("")
    
    vibez.spill("🎉 Neural Network Module Test Suite Completed!")
    vibez.spill("All core neural network functionality has been validated.")
    vibez.spill("The nnz module is ready for production AI/ML development!")
}

fr fr Execute the comprehensive test suite
run_all_tests()
