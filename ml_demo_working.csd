yeet "mathz"

fr fr Working Machine Learning Demo - Basic Concepts

vibez.spill("=== CURSED Machine Learning Demo ===")

fr fr 1. Basic statistical operations
vibez.spill("\n1. Statistical Operations")
sus data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
vibez.spill("Data: [1.0, 2.0, 3.0, 4.0, 5.0]")

fr fr Calculate mean
sus sum meal = 0.0
sus i normie = 0
bestie i < len(data) {
    sum = sum + data[i]
    i = i + 1
}
sus mean meal = sum / len(data)
vibez.spill("Mean: ", mean)

fr fr Calculate standard deviation
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

fr fr 2. Activation functions
vibez.spill("\n2. Activation Functions")

fr fr Sigmoid function
slay sigmoid_func(x meal) meal {
    lowkey x > 500.0 {
        damn 1.0
    }
    lowkey x < -500.0 {
        damn 0.0
    }
    damn 1.0 / (1.0 + exp_meal(-x))
}

fr fr ReLU function
slay relu_func(x meal) meal {
    lowkey x > 0.0 {
        damn x
    }
    damn 0.0
}

vibez.spill("Sigmoid(0.0) = ", sigmoid_func(0.0))
vibez.spill("Sigmoid(2.0) = ", sigmoid_func(2.0))
vibez.spill("ReLU(-1.0) = ", relu_func(-1.0))
vibez.spill("ReLU(3.0) = ", relu_func(3.0))

fr fr 3. Loss functions
vibez.spill("\n3. Loss Functions")

fr fr Mean squared error
slay mse_func(pred []meal, actual []meal) meal {
    lowkey len(pred) != len(actual) {
        damn 0.0
    }
    sus sum_error meal = 0.0
    sus i normie = 0
    bestie i < len(pred) {
        sus error meal = pred[i] - actual[i]
        sum_error = sum_error + error * error
        i = i + 1
    }
    damn sum_error / len(pred)
}

sus predictions []meal = [2.5, 0.0, 2.1]
sus targets []meal = [3.0, -0.5, 2.0]
sus mse meal = mse_func(predictions, targets)
vibez.spill("Predictions: [2.5, 0.0, 2.1]")
vibez.spill("Targets: [3.0, -0.5, 2.0]")
vibez.spill("MSE Loss: ", mse)

fr fr 4. Simple linear regression training
vibez.spill("\n4. Linear Regression Training")

sus x_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus y_data []meal = [3.1, 5.0, 7.2, 8.9, 11.0]

vibez.spill("Training data (y ≈ 2x + 1):")
i = 0
bestie i < len(x_data) {
    vibez.spill("x=", x_data[i], ", y=", y_data[i])
    i = i + 1
}

fr fr Model parameters
sus weight meal = 0.1
sus bias meal = 0.0
sus learning_rate meal = 0.01

fr fr Training function
slay train_step(x []meal, y []meal, w meal, b meal, lr meal) (meal, meal) {
    fr fr Forward pass
    sus pred_sum meal = 0.0
    sus loss_sum meal = 0.0
    
    fr fr Calculate predictions and loss
    sus j normie = 0
    bestie j < len(x) {
        sus pred meal = w * x[j] + b
        sus error meal = pred - y[j]
        loss_sum = loss_sum + error * error
        j = j + 1
    }
    
    fr fr Calculate gradients
    sus w_grad meal = 0.0
    sus b_grad meal = 0.0
    
    j = 0
    bestie j < len(x) {
        sus pred meal = w * x[j] + b
        sus error meal = pred - y[j]
        w_grad = w_grad + error * x[j]
        b_grad = b_grad + error
        j = j + 1
    }
    
    w_grad = w_grad / len(x)
    b_grad = b_grad / len(x)
    
    fr fr Update parameters
    sus new_w meal = w - lr * w_grad
    sus new_b meal = b - lr * b_grad
    
    damn (new_w, new_b)
}

vibez.spill("\nTraining...")

fr fr Training loop
sus epoch normie = 0
bestie epoch < 100 {
    sus new_weight meal
    sus new_bias meal
    (new_weight, new_bias) = train_step(x_data, y_data, weight, bias, learning_rate)
    weight = new_weight
    bias = new_bias
    
    lowkey epoch % 20 == 0 {
        fr fr Calculate current loss
        sus current_loss meal = 0.0
        sus k normie = 0
        bestie k < len(x_data) {
            sus pred meal = weight * x_data[k] + bias
            sus error meal = pred - y_data[k]
            current_loss = current_loss + error * error
            k = k + 1
        }
        current_loss = current_loss / len(x_data)
        vibez.spill("Epoch ", epoch, ": Loss = ", current_loss)
    }
    epoch = epoch + 1
}

vibez.spill("\nFinal parameters:")
vibez.spill("Weight: ", weight)
vibez.spill("Bias: ", bias)

fr fr Test predictions
vibez.spill("\nPredictions vs Actual:")
i = 0
bestie i < len(x_data) {
    sus pred meal = weight * x_data[i] + bias
    vibez.spill("x=", x_data[i], " -> pred=", pred, ", actual=", y_data[i])
    i = i + 1
}

fr fr 5. Simple clustering
vibez.spill("\n5. Simple Clustering")

sus points []meal = [1.0, 1.2, 5.0, 5.1]  fr fr 1D points for simplicity

fr fr Simple k-means (k=2)
sus cluster1_center meal = 2.0  fr fr Initial guess
sus cluster2_center meal = 4.0  fr fr Initial guess

fr fr One iteration of k-means
sus cluster1_points []meal = []
sus cluster2_points []meal = []

i = 0
bestie i < len(points) {
    sus dist1 meal = abs_meal(points[i] - cluster1_center)
    sus dist2 meal = abs_meal(points[i] - cluster2_center)
    
    lowkey dist1 < dist2 {
        cluster1_points = append(cluster1_points, points[i])
    } {
        cluster2_points = append(cluster2_points, points[i])
    }
    i = i + 1
}

fr fr Update cluster centers
lowkey len(cluster1_points) > 0 {
    sus sum1 meal = 0.0
    i = 0
    bestie i < len(cluster1_points) {
        sum1 = sum1 + cluster1_points[i]
        i = i + 1
    }
    cluster1_center = sum1 / len(cluster1_points)
}

lowkey len(cluster2_points) > 0 {
    sus sum2 meal = 0.0
    i = 0
    bestie i < len(cluster2_points) {
        sum2 = sum2 + cluster2_points[i]
        i = i + 1
    }
    cluster2_center = sum2 / len(cluster2_points)
}

vibez.spill("Points: [1.0, 1.2, 5.0, 5.1]")
vibez.spill("Cluster 1 center: ", cluster1_center)
vibez.spill("Cluster 2 center: ", cluster2_center)

fr fr 6. Matrix multiplication (2x2)
vibez.spill("\n6. Matrix Operations")

slay matrix_mult_2x2(a []meal, b []meal) []meal {
    sus result []meal = [0.0, 0.0, 0.0, 0.0]
    
    fr fr Manual 2x2 matrix multiplication
    result[0] = a[0] * b[0] + a[1] * b[2]  fr fr result[0,0]
    result[1] = a[0] * b[1] + a[1] * b[3]  fr fr result[0,1]
    result[2] = a[2] * b[0] + a[3] * b[2]  fr fr result[1,0]
    result[3] = a[2] * b[1] + a[3] * b[3]  fr fr result[1,1]
    
    damn result
}

sus matrix_a []meal = [1.0, 2.0, 3.0, 4.0]  fr fr [[1,2], [3,4]]
sus matrix_b []meal = [5.0, 6.0, 7.0, 8.0]  fr fr [[5,6], [7,8]]
sus result []meal = matrix_mult_2x2(matrix_a, matrix_b)

vibez.spill("Matrix A: [[1,2], [3,4]]")
vibez.spill("Matrix B: [[5,6], [7,8]]")
vibez.spill("A * B = [[", result[0], ",", result[1], "], [", result[2], ",", result[3], "]]")

fr fr 7. Normalization
vibez.spill("\n7. Data Normalization")

sus raw_data []meal = [10.0, 20.0, 30.0, 40.0, 50.0]

fr fr Min-max normalization
sus min_val meal = raw_data[0]
sus max_val meal = raw_data[0]

i = 1
bestie i < len(raw_data) {
    lowkey raw_data[i] < min_val {
        min_val = raw_data[i]
    }
    lowkey raw_data[i] > max_val {
        max_val = raw_data[i]
    }
    i = i + 1
}

sus range meal = max_val - min_val
sus normalized []meal = []

i = 0
bestie i < len(raw_data) {
    sus norm_val meal = (raw_data[i] - min_val) / range
    normalized = append(normalized, norm_val)
    i = i + 1
}

vibez.spill("Raw data: [10, 20, 30, 40, 50]")
vibez.spill("Normalized: [", normalized[0], ", ", normalized[1], ", ", normalized[2], ", ", normalized[3], ", ", normalized[4], "]")

vibez.spill("\n=== Demo Complete ===")
vibez.spill("Demonstrated ML building blocks:")
vibez.spill("✓ Statistics (mean, std dev)")
vibez.spill("✓ Activation functions (sigmoid, ReLU)")
vibez.spill("✓ Loss functions (MSE)")
vibez.spill("✓ Linear regression with gradient descent")
vibez.spill("✓ K-means clustering")
vibez.spill("✓ Matrix operations")
vibez.spill("✓ Data normalization")
