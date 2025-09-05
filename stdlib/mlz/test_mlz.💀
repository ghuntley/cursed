yeet "testz"
yeet "mlz"
yeet "tensorz"
yeet "mathz"

fr fr Simplified Machine Learning Test Suite

test_start("activation_functions")

fr fr Test sigmoid activation
assert_near(sigmoid(0.0), 0.5, 0.001)
assert_near(sigmoid(1000.0), 1.0, 0.001)  fr fr Large positive should be ~1
assert_near(sigmoid(-1000.0), 0.0, 0.001) fr fr Large negative should be ~0

fr fr Test sigmoid derivative
assert_near(sigmoid_derivative(0.0), 0.25, 0.001)  fr fr 0.5 * (1 - 0.5) = 0.25

fr fr Test ReLU activation
assert_near(relu(5.0), 5.0, 0.001)
assert_near(relu(-3.0), 0.0, 0.001)
assert_near(relu(0.0), 0.0, 0.001)

fr fr Test ReLU derivative
assert_near(relu_derivative(5.0), 1.0, 0.001)
assert_near(relu_derivative(-3.0), 0.0, 0.001)
assert_near(relu_derivative(0.0), 0.0, 0.001)

fr fr Test tanh activation
assert_near(tanh_activation(0.0), 0.0, 0.001)

fr fr Test leaky ReLU
assert_near(leaky_relu(5.0, 0.1), 5.0, 0.001)
assert_near(leaky_relu(-2.0, 0.1), -0.2, 0.001)  fr fr -2 * 0.1

test_start("loss_functions")

fr fr Test MSE loss
sus pred_data meal[value] = [1.0, 2.0, 3.0]
sus target_data meal[value] = [1.5, 2.5, 2.5]
sus mse meal = mse_loss(pred_data, target_data)
fr fr Expected: ((1-1.5)² + (2-2.5)² + (3-2.5)²) / 3 = (0.25 + 0.25 + 0.25) / 3 = 0.25
assert_near(mse, 0.25, 0.001)

fr fr Test MAE loss
sus mae meal = mae_loss(pred_data, target_data)
fr fr Expected: (|1-1.5| + |2-2.5| + |3-2.5|) / 3 = (0.5 + 0.5 + 0.5) / 3 = 0.5
assert_near(mae, 0.5, 0.001)

fr fr Test binary cross-entropy loss
sus binary_pred_data meal[value] = [0.8, 0.3, 0.9]
sus binary_target_data meal[value] = [1.0, 0.0, 1.0]
sus bce meal = binary_crossentropy_loss(binary_pred_data, binary_target_data)
fr fr Should be a positive value for this case
assert_true(bce > 0.0)
assert_true(bce < 10.0)  fr fr Reasonable range

test_start("linear_regression_basics")

fr fr Test weight initialization
sus weights meal[value] = linear_model_weights_init(3)
assert_eq_int(len(weights), 3)

fr fr Test single prediction
sus features meal[value] = [1.0, 2.0, 3.0]
sus test_weights meal[value] = [0.5, 1.0, 1.5]
sus bias meal = 0.1
sus prediction meal = linear_model_predict_single(features, test_weights, bias)
fr fr Expected: 1*0.5 + 2*1.0 + 3*1.5 + 0.1 = 0.5 + 2.0 + 4.5 + 0.1 = 7.1
assert_near(prediction, 7.1, 0.001)

fr fr Test batch prediction
sus feature_matrix meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]  fr fr 2 samples, 3 features each
sus batch_predictions meal[value] = linear_model_predict_batch(feature_matrix, 2, 3, test_weights, bias)
assert_eq_int(len(batch_predictions), 2)
fr fr First sample: [1,2,3] -> 1*0.5 + 2*1.0 + 3*1.5 + 0.1 = 7.1
assert_near(batch_predictions[0], 7.1, 0.001)
fr fr Second sample: [4,5,6] -> 4*0.5 + 5*1.0 + 6*1.5 + 0.1 = 2.0 + 5.0 + 9.0 + 0.1 = 16.1
assert_near(batch_predictions[1], 16.1, 0.001)

test_start("linear_regression_training")

fr fr Test simple linear regression training step
sus train_features meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]  fr fr [[1,2], [3,4], [5,6]]
sus train_targets meal[value] = [3.0, 7.0, 11.0]  fr fr y = x1 + x2
sus initial_weights meal[value] = [0.1, 0.1]
sus initial_bias meal = 0.0

sus updated_weights meal[value]
sus updated_bias meal
(updated_weights, updated_bias) = linear_model_train_step(train_features, train_targets, initial_weights, initial_bias, 3, 2, 0.01)

assert_eq_int(len(updated_weights), 2)
fr fr Weights should have changed during training
sus weights_changed lit = cringe
lowkey abs_meal(updated_weights[0] - initial_weights[0]) > 0.001 {
    weights_changed = based
}
lowkey abs_meal(updated_weights[1] - initial_weights[1]) > 0.001 {
    weights_changed = based
}
assert_true(weights_changed)

test_start("neural_network_layer")

fr fr Test layer weight initialization
sus layer_weights meal[value] = layer_weights_init(3, 2)
assert_eq_int(len(layer_weights), 6)  fr fr 3 * 2

fr fr Test layer bias initialization
sus layer_biases meal[value] = layer_biases_init(2)
assert_eq_int(len(layer_biases), 2)
assert_near(layer_biases[0], 0.0, 0.001)
assert_near(layer_biases[1], 0.0, 0.001)

fr fr Test layer forward pass with known weights
sus input meal[value] = [1.0, 2.0, 3.0]
sus known_weights meal[value] = [1.0, 0.0, 1.0, 0.0, 1.0, 1.0]  fr fr 3x2 matrix
sus known_biases meal[value] = [0.0, 1.0]
sus output meal[value] = layer_forward_single(input, known_weights, known_biases, 3, 2)

assert_eq_int(len(output), 2)
fr fr Output[0] = 1*1 + 2*1 + 3*1 + 0 = 6
fr fr Output[1] = 1*0 + 2*0 + 3*1 + 1 = 4
assert_near(output[0], 6.0, 0.001)
assert_near(output[1], 4.0, 0.001)

test_start("gradient_descent_optimizers")

fr fr Test SGD weight update
sus original_weights meal[value] = [1.0, 2.0, 3.0]
sus gradients meal[value] = [0.1, 0.2, 0.3]
sus new_weights meal[value] = sgd_update_weights(original_weights, gradients, 0.1)

assert_eq_int(len(new_weights), 3)
fr fr new_weight = old_weight - learning_rate * gradient
assert_near(new_weights[0], 1.0 - 0.1 * 0.1, 0.001)  fr fr 1.0 - 0.01 = 0.99
assert_near(new_weights[1], 2.0 - 0.1 * 0.2, 0.001)  fr fr 2.0 - 0.02 = 1.98
assert_near(new_weights[2], 3.0 - 0.1 * 0.3, 0.001)  fr fr 3.0 - 0.03 = 2.97

fr fr Test momentum update
sus momentum_weights meal[value] = [0.0, 0.0, 0.0]
sus updated_weights meal[value]
sus updated_momentum meal[value]
(updated_weights, updated_momentum) = momentum_update_weights(original_weights, gradients, momentum_weights, 0.1, 0.9)

assert_eq_int(len(updated_weights), 3)
assert_eq_int(len(updated_momentum), 3)

test_start("kmeans_clustering")

fr fr Test centroid initialization
set_random_seed(42)  fr fr For reproducible results
sus kmeans_data meal[value] = [1.0, 1.0, 1.1, 1.1, 5.0, 5.0, 5.1, 5.1]  fr fr 4 samples, 2 features
sus initial_centroids meal[value] = kmeans_init_centroids(kmeans_data, 4, 2, 2)
assert_eq_int(len(initial_centroids), 4)  fr fr 2 centroids * 2 features

fr fr Test cluster assignment
sus assignments normie[value] = kmeans_assign_clusters(kmeans_data, initial_centroids, 4, 2, 2)
assert_eq_int(len(assignments), 4)

fr fr All assignments should be valid cluster indices (0 or 1)
sus i normie = 0
bestie i < len(assignments) {
    sus cluster normie = assignments[i]
    assert_true(cluster == 0 || cluster == 1)
    i = i + 1
}

fr fr Test centroid update
sus updated_centroids meal[value] = kmeans_update_centroids(kmeans_data, assignments, 4, 2, 2)
assert_eq_int(len(updated_centroids), 4)  fr fr 2 centroids * 2 features

test_start("evaluation_metrics")

fr fr Test accuracy score
sus acc_pred_data meal[value] = [0.8, 0.3, 0.9, 0.1]
sus acc_target_data meal[value] = [1.0, 0.0, 1.0, 0.0]
sus accuracy meal = accuracy_score(acc_pred_data, acc_target_data, 0.5)
fr fr All predictions are correct: 0.8>0.5->1 matches 1, 0.3<0.5->0 matches 0, etc.
assert_near(accuracy, 1.0, 0.001)

fr fr Test R² score
sus r2_pred_data meal[value] = [2.0, 4.0, 6.0, 8.0]
sus r2_target_data meal[value] = [2.1, 3.9, 6.1, 7.9]
sus r2 meal = r2_score(r2_pred_data, r2_target_data)
fr fr Should be close to 1 for good predictions
assert_true(r2 > 0.8)
assert_true(r2 <= 1.0)

test_start("data_preprocessing")

fr fr Test train-test split indices
sus train_indices normie[value]
sus test_indices normie[value]
(train_indices, test_indices) = train_test_split_indices(10, 0.2)
assert_eq_int(len(train_indices), 8)  fr fr 80% training
assert_eq_int(len(test_indices), 2)   fr fr 20% testing

fr fr Test sample extraction by indices
sus sample_data meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]  fr fr 3 samples, 2 features
sus indices normie[value] = [0, 2]
sus extracted_samples meal[value] = extract_samples_by_indices(sample_data, indices, 2)
assert_eq_int(len(extracted_samples), 4)  fr fr 2 samples * 2 features
assert_near(extracted_samples[0], 1.0, 0.001)  fr fr First sample, first feature
assert_near(extracted_samples[1], 2.0, 0.001)  fr fr First sample, second feature
assert_near(extracted_samples[2], 5.0, 0.001)  fr fr Third sample, first feature
assert_near(extracted_samples[3], 6.0, 0.001)  fr fr Third sample, second feature

fr fr Test label extraction by indices
sus labels meal[value] = [10.0, 20.0, 30.0]
sus extracted_labels meal[value] = extract_labels_by_indices(labels, indices)
assert_eq_int(len(extracted_labels), 2)
assert_near(extracted_labels[0], 10.0, 0.001)  fr fr First label
assert_near(extracted_labels[1], 30.0, 0.001)  fr fr Third label

test_start("feature_standardization")

fr fr Test feature standardization
sus std_features meal[value] = [1.0, 10.0, 2.0, 20.0, 3.0, 30.0]  fr fr 3 samples, 2 features
sus standardized meal[value]
sus means meal[value]
sus stds meal[value]
(standardized, means, stds) = standardize_features_simple(std_features, 3, 2)

fr fr Check means and standard deviations
fr fr Feature 0: [1, 2, 3] -> mean=2
fr fr Feature 1: [10, 20, 30] -> mean=20
assert_near(means[0], 2.0, 0.001)
assert_near(means[1], 20.0, 0.001)

fr fr Check that standardized features have reasonable values
assert_eq_int(len(standardized), 6)  fr fr 3 samples * 2 features

test_start("utility_functions")

fr fr Test data shuffling
sus shuffled_indices normie[value] = shuffle_data_indices(5)
assert_eq_int(len(shuffled_indices), 5)

fr fr Check that all indices 0-4 are present
sus found_indices lit[value] = [cringe, cringe, cringe, cringe, cringe]
sus i normie = 0
bestie i < len(shuffled_indices) {
    sus index normie = shuffled_indices[i]
    lowkey index >= 0 && index < 5 {
        found_indices[index] = based
    }
    i = i + 1
}

fr fr All indices should be found
i = 0
bestie i < 5 {
    assert_true(found_indices[i])
    i = i + 1
}

fr fr Test batch creation
sus batches normie[value][value] = create_batches(7, 3)
assert_eq_int(len(batches), 3)  fr fr Batches: [0,1,2], [3,4,5], [6]
assert_eq_int(len(batches[0]), 3)  fr fr First batch size
assert_eq_int(len(batches[1]), 3)  fr fr Second batch size
assert_eq_int(len(batches[2]), 1)  fr fr Last batch size

test_start("integration_workflow")

fr fr Test a simple end-to-end ML workflow
fr fr Create simple dataset where y = 2*x1 + 3*x2 + noise
sus workflow_features meal[value] = [1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0]  fr fr 4 samples, 2 features
sus workflow_targets meal[value] = [5.1, 10.2, 15.1, 20.1]  fr fr Approximately 2*x1 + 3*x2

fr fr Initialize model
sus workflow_weights meal[value] = linear_model_weights_init(2)
sus workflow_bias meal = 0.0

fr fr Train for a few steps
sus step normie = 0
bestie step < 10 {
    (workflow_weights, workflow_bias) = linear_model_train_step(workflow_features, workflow_targets, workflow_weights, workflow_bias, 4, 2, 0.01)
    step = step + 1
}

fr fr Check that model learned something
sus final_predictions meal[value] = linear_model_predict_batch(workflow_features, 4, 2, workflow_weights, workflow_bias)
sus final_loss meal = mse_loss(final_predictions, workflow_targets)
fr fr Loss should be reasonable (not infinite or extremely large)
assert_true(final_loss >= 0.0)
assert_true(final_loss < 100.0)

test_start("edge_cases_ml")

fr fr Test edge cases for ML functions

fr fr Empty arrays in loss functions
sus empty_pred meal[value] = []
sus empty_target meal[value] = []
sus empty_mse meal = mse_loss(empty_pred, empty_target)
assert_near(empty_mse, 0.0, 0.001)

fr fr Single sample scenarios
sus single_pred meal[value] = [5.0]
sus single_target meal[value] = [4.0]
sus single_mse meal = mse_loss(single_pred, single_target)
assert_near(single_mse, 1.0, 0.001)  fr fr (5-4)² = 1

fr fr Extreme activation values
sus extreme_sigmoid meal = sigmoid(1000.0)
assert_true(extreme_sigmoid >= 0.0)
assert_true(extreme_sigmoid <= 1.0)

sus extreme_relu meal = relu(-1000.0)
assert_near(extreme_relu, 0.0, 0.001)

print_test_summary()
