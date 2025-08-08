yeet "mathz"

fr fr Basic Machine Learning Demo without complex modules

vibez.spill("=== Basic Machine Learning Concepts Demo ===")

fr fr 1. Basic mathematical operations for ML
vibez.spill("\n1. Mathematical Operations")
sus data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]

fr fr Calculate mean (average)
sus sum meal = 0.0
sus i normie = 0
bestie i < len(data) {
    sum = sum + data[i]
    i = i + 1
}
sus mean meal = sum / len(data)
vibez.spill("Data: [1.0, 2.0, 3.0, 4.0, 5.0]")
vibez.spill("Mean: ", mean)

fr fr Calculate variance
sus variance_sum meal = 0.0
i = 0
bestie i < len(data) {
    sus diff meal = data[i] - mean
    variance_sum = variance_sum + diff * diff
    i = i + 1
}
sus variance meal = variance_sum / (len(data) - 1)
sus std_dev meal = sqrt_meal(variance)
vibez.spill("Standard Deviation: ", std_dev)

fr fr 2. Activation Functions
vibez.spill("\n2. Activation Functions")

fr fr Sigmoid function: 1 / (1 + e^(-x))
sus test_input meal = 0.0
sus sigmoid_result meal = 1.0 / (1.0 + exp_meal(-test_input))
vibez.spill("Sigmoid(0.0) = ", sigmoid_result)

test_input = 2.0
sigmoid_result = 1.0 / (1.0 + exp_meal(-test_input))
vibez.spill("Sigmoid(2.0) = ", sigmoid_result)

fr fr ReLU function: max(0, x)
test_input = -1.0
sus relu_result meal = max_meal(0.0, test_input)
vibez.spill("ReLU(-1.0) = ", relu_result)

test_input = 3.0
relu_result = max_meal(0.0, test_input)
vibez.spill("ReLU(3.0) = ", relu_result)

fr fr 3. Loss Functions
vibez.spill("\n3. Loss Functions")

fr fr Mean Squared Error
sus predictions []meal = [2.5, 0.0, 2.1]
sus actual []meal = [3.0, -0.5, 2.0]

sus mse_sum meal = 0.0
i = 0
bestie i < len(predictions) {
    sus error meal = predictions[i] - actual[i]
    mse_sum = mse_sum + error * error
    i = i + 1
}
sus mse meal = mse_sum / len(predictions)
vibez.spill("Predictions: [2.5, 0.0, 2.1]")
vibez.spill("Actual: [3.0, -0.5, 2.0]")
vibez.spill("Mean Squared Error: ", mse)

fr fr 4. Simple Linear Regression (y = wx + b)
vibez.spill("\n4. Simple Linear Regression")

fr fr Training data: y = 2x + 1 (with some noise)
sus x_train []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus y_train []meal = [3.1, 5.0, 7.2, 8.9, 11.0]

fr fr Initialize parameters
sus weight meal = 0.1  fr fr w
sus bias meal = 0.0    fr fr b
sus learning_rate meal = 0.01

vibez.spill("Training data:")
i = 0
bestie i < len(x_train) {
    vibez.spill("x=", x_train[i], ", y=", y_train[i])
    i = i + 1
}

vibez.spill("\nTraining...")

fr fr Simple gradient descent training
sus epoch normie = 0
bestie epoch < 100 {
    fr fr Forward pass: calculate predictions
    sus predictions_new []meal = []
    i = 0
    bestie i < len(x_train) {
        sus prediction meal = weight * x_train[i] + bias
        predictions_new = append(predictions_new, prediction)
        i = i + 1
    }
    
    fr fr Calculate loss (MSE)
    sus loss_sum meal = 0.0
    i = 0
    bestie i < len(predictions_new) {
        sus error meal = predictions_new[i] - y_train[i]
        loss_sum = loss_sum + error * error
        i = i + 1
    }
    sus current_loss meal = loss_sum / len(predictions_new)
    
    fr fr Calculate gradients
    sus weight_gradient meal = 0.0
    sus bias_gradient meal = 0.0
    
    i = 0
    bestie i < len(predictions_new) {
        sus error meal = predictions_new[i] - y_train[i]
        weight_gradient = weight_gradient + error * x_train[i]
        bias_gradient = bias_gradient + error
        i = i + 1
    }
    
    weight_gradient = weight_gradient / len(predictions_new)
    bias_gradient = bias_gradient / len(predictions_new)
    
    fr fr Update parameters
    weight = weight - learning_rate * weight_gradient
    bias = bias - learning_rate * bias_gradient
    
    fr fr Print progress every 20 epochs
    lowkey epoch % 20 == 0 {
        vibez.spill("Epoch ", epoch, ": Loss = ", current_loss, ", Weight = ", weight, ", Bias = ", bias)
    }
    
    epoch = epoch + 1
}

vibez.spill("\nFinal model parameters:")
vibez.spill("Weight: ", weight)
vibez.spill("Bias: ", bias)
vibez.spill("Learned equation: y = ", weight, "x + ", bias)

fr fr Test the trained model
vibez.spill("\nTesting the model:")
i = 0
bestie i < len(x_train) {
    sus predicted meal = weight * x_train[i] + bias
    vibez.spill("x=", x_train[i], " -> predicted=", predicted, ", actual=", y_train[i])
    i = i + 1
}

fr fr 5. Simple clustering (2D points)
vibez.spill("\n5. Simple 2-Point Clustering")

fr fr Two clusters of points
sus points_x []meal = [1.0, 1.2, 5.0, 5.1]
sus points_y []meal = [1.0, 1.1, 5.0, 5.2]

fr fr Simple centroid calculation for each cluster
fr fr Cluster 1: points 0,1  Cluster 2: points 2,3
sus cluster1_center_x meal = (points_x[0] + points_x[1]) / 2.0
sus cluster1_center_y meal = (points_y[0] + points_y[1]) / 2.0
sus cluster2_center_x meal = (points_x[2] + points_x[3]) / 2.0
sus cluster2_center_y meal = (points_y[2] + points_y[3]) / 2.0

vibez.spill("Points:")
i = 0
bestie i < len(points_x) {
    vibez.spill("Point ", i, ": (", points_x[i], ", ", points_y[i], ")")
    i = i + 1
}

vibez.spill("Cluster 1 center: (", cluster1_center_x, ", ", cluster1_center_y, ")")
vibez.spill("Cluster 2 center: (", cluster2_center_x, ", ", cluster2_center_y, ")")

fr fr 6. Matrix multiplication example
vibez.spill("\n6. Matrix Multiplication")

fr fr 2x2 matrices
sus matrix_a []meal = [1.0, 2.0, 3.0, 4.0]  fr fr [[1,2], [3,4]]
sus matrix_b []meal = [5.0, 6.0, 7.0, 8.0]  fr fr [[5,6], [7,8]]

vibez.spill("Matrix A: [[1,2], [3,4]]")
vibez.spill("Matrix B: [[5,6], [7,8]]")

fr fr Calculate A * B manually
fr fr Result[0,0] = A[0,0]*B[0,0] + A[0,1]*B[1,0] = 1*5 + 2*7 = 19
fr fr Result[0,1] = A[0,0]*B[0,1] + A[0,1]*B[1,1] = 1*6 + 2*8 = 22
fr fr Result[1,0] = A[1,0]*B[0,0] + A[1,1]*B[1,0] = 3*5 + 4*7 = 43
fr fr Result[1,1] = A[1,0]*B[0,1] + A[1,1]*B[1,1] = 3*6 + 4*8 = 50

sus result_00 meal = matrix_a[0] * matrix_b[0] + matrix_a[1] * matrix_b[2]
sus result_01 meal = matrix_a[0] * matrix_b[1] + matrix_a[1] * matrix_b[3]
sus result_10 meal = matrix_a[2] * matrix_b[0] + matrix_a[3] * matrix_b[2]
sus result_11 meal = matrix_a[2] * matrix_b[1] + matrix_a[3] * matrix_b[3]

vibez.spill("A * B = [[", result_00, ",", result_01, "], [", result_10, ",", result_11, "]]")

vibez.spill("\n=== Demo Complete ===")
vibez.spill("Successfully demonstrated core ML concepts:")
vibez.spill("✓ Statistical operations (mean, variance, std dev)")
vibez.spill("✓ Activation functions (sigmoid, ReLU)")
vibez.spill("✓ Loss functions (MSE)")
vibez.spill("✓ Linear regression with gradient descent")
vibez.spill("✓ Simple clustering")
vibez.spill("✓ Matrix operations")
vibez.spill("\nThese are the foundational building blocks for:")
vibez.spill("- Neural networks")
vibez.spill("- Deep learning")
vibez.spill("- Machine learning algorithms")
vibez.spill("- Data preprocessing")
