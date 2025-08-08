yeet "tensorz"
yeet "mlz"
yeet "mathz"

fr fr Simple Machine Learning Demo using CURSED primitives

vibez.spill("=== CURSED Machine Learning Demo ===")

fr fr 1. Create and test tensors (1D arrays)
vibez.spill("\n1. Tensor Operations")
sus data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus tensor []meal = tensor_from_array_1d(data)
vibez.spill("Original tensor: ")
tensor_print_1d(tensor, "tensor")

sus mean_val meal = tensor_mean_1d(tensor)
sus std_val meal = tensor_std_dev_1d(tensor)
vibez.spill("Mean: ", mean_val)
vibez.spill("Std Dev: ", std_val)

fr fr 2. Test activation functions
vibez.spill("\n2. Activation Functions")
sus sigmoid_input []meal = [-2.0, -1.0, 0.0, 1.0, 2.0]
sus sigmoid_output []meal = apply_sigmoid_1d(sigmoid_input)
vibez.spill("Sigmoid activation: ")
tensor_print_1d(sigmoid_output, "sigmoid")

sus relu_input []meal = [-2.0, -1.0, 0.0, 1.0, 2.0]
sus relu_output []meal = apply_relu_1d(relu_input)
vibez.spill("ReLU activation: ")
tensor_print_1d(relu_output, "relu")

fr fr 3. Test loss functions
vibez.spill("\n3. Loss Functions")
sus predictions []meal = [1.0, 2.0, 3.0]
sus targets []meal = [1.2, 2.1, 2.9]
sus mse meal = mse_loss(predictions, targets)
sus mae meal = mae_loss(predictions, targets)
vibez.spill("MSE Loss: ", mse)
vibez.spill("MAE Loss: ", mae)

fr fr 4. Linear regression demo
vibez.spill("\n4. Linear Regression Demo")
fr fr Create simple dataset: y = 2*x + 1 + noise
sus x_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]  fr fr 5 samples, 1 feature each
sus y_data []meal = [3.1, 5.2, 7.0, 8.9, 11.1]  fr fr y ≈ 2*x + 1

fr fr Initialize model
sus weights []meal = linear_model_weights_init(1)
sus bias meal = 0.0
vibez.spill("Initial weight: ", weights[0])
vibez.spill("Initial bias: ", bias)

fr fr Train for a few epochs
sus epoch normie = 0
bestie epoch < 50 {
    sus updated_weights []meal
    sus updated_bias meal
    (updated_weights, updated_bias) = linear_model_train_step(x_data, y_data, weights, bias, 5, 1, 0.01)
    weights = updated_weights
    bias = updated_bias
    
    lowkey epoch % 10 == 0 {
        sus current_predictions []meal = linear_model_predict_batch(x_data, 5, 1, weights, bias)
        sus current_loss meal = mse_loss(current_predictions, y_data)
        vibez.spill("Epoch ", epoch, ": Loss = ", current_loss)
    }
    epoch = epoch + 1
}

vibez.spill("Final weight: ", weights[0])
vibez.spill("Final bias: ", bias)

fr fr Make final predictions
sus final_predictions []meal = linear_model_predict_batch(x_data, 5, 1, weights, bias)
vibez.spill("Final predictions: ")
tensor_print_1d(final_predictions, "predictions")
vibez.spill("Actual targets: ")
tensor_print_1d(y_data, "targets")

fr fr 5. Matrix operations demo
vibez.spill("\n5. Matrix Operations")
sus matrix_a []meal = [1.0, 2.0, 3.0, 4.0]  fr fr 2x2 matrix
sus matrix_b []meal = [5.0, 6.0, 7.0, 8.0]  fr fr 2x2 matrix

vibez.spill("Matrix A: ")
matrix_print(matrix_a, 2, 2, "A")
vibez.spill("Matrix B: ")
matrix_print(matrix_b, 2, 2, "B")

sus matrix_product []meal = matrix_multiply(matrix_a, 2, 2, matrix_b, 2, 2)
vibez.spill("A * B: ")
matrix_print(matrix_product, 2, 2, "A*B")

fr fr 6. K-means clustering demo
vibez.spill("\n6. K-Means Clustering Demo")
set_random_seed(42)  fr fr For reproducible results
sus cluster_data []meal = [1.0, 1.0, 1.2, 1.1, 5.0, 5.0, 5.1, 5.2]  fr fr 4 points, 2 features
sus centroids []meal
sus assignments []normie
(centroids, assignments) = kmeans_cluster(cluster_data, 4, 2, 2, 10)

vibez.spill("Cluster assignments:")
sus i normie = 0
bestie i < len(assignments) {
    vibez.spill("Point ", i, ": Cluster ", assignments[i])
    i = i + 1
}

fr fr 7. Evaluation metrics demo
vibez.spill("\n7. Evaluation Metrics")
sus binary_predictions []meal = [0.9, 0.2, 0.8, 0.1]
sus binary_targets []meal = [1.0, 0.0, 1.0, 0.0]
sus accuracy meal = accuracy_score(binary_predictions, binary_targets, 0.5)
vibez.spill("Binary classification accuracy: ", accuracy * 100.0, "%")

sus regression_r2 meal = r2_score(final_predictions, y_data)
vibez.spill("Regression R² score: ", regression_r2)

vibez.spill("\n=== Demo Complete ===")
vibez.spill("Successfully demonstrated:")
vibez.spill("✓ Tensor operations and statistics")
vibez.spill("✓ Activation functions (sigmoid, ReLU)")
vibez.spill("✓ Loss functions (MSE, MAE)")
vibez.spill("✓ Linear regression training")
vibez.spill("✓ Matrix operations and multiplication")
vibez.spill("✓ K-means clustering")
vibez.spill("✓ Evaluation metrics")
