fr fr CURSED Machine Learning Module - Simplified Pure CURSED Implementation
fr fr Compatible with current parser capabilities

yeet "mathz"
yeet "tensorz"
yeet "arrayz"

fr fr === ACTIVATION FUNCTIONS ===

slay sigmoid(x meal) meal {
    lowkey x > 500.0 {
        damn 1.0
    }
    lowkey x < -500.0 {
        damn 0.0
    }
    sus exp_neg_x meal = exp_meal(-x)
    damn 1.0 / (1.0 + exp_neg_x)
}

slay sigmoid_derivative(x meal) meal {
    sus s meal = sigmoid(x)
    damn s * (1.0 - s)
}

slay relu(x meal) meal {
    lowkey x > 0.0 {
        damn x
    }
    damn 0.0
}

slay relu_derivative(x meal) meal {
    lowkey x > 0.0 {
        damn 1.0
    }
    damn 0.0
}

slay tanh_activation(x meal) meal {
    damn tanh_meal(x)
}

slay tanh_derivative(x meal) meal {
    sus t meal = tanh_meal(x)
    damn 1.0 - t * t
}

slay leaky_relu(x meal, alpha meal) meal {
    lowkey x > 0.0 {
        damn x
    }
    damn alpha * x
}

slay softmax_single(x meal, sum_exp meal) meal {
    damn exp_meal(x) / sum_exp
}

fr fr === LOSS FUNCTIONS ===

slay mse_loss(predictions []meal, targets []meal) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus sum_squared_error meal = 0.0
    sus i normie = 0
    bestie i < len(predictions) {
        sus diff meal = predictions[i] - targets[i]
        sum_squared_error = sum_squared_error + diff * diff
        i = i + 1
    }
    damn sum_squared_error / len(predictions)
}

slay mae_loss(predictions []meal, targets []meal) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus sum_absolute_error meal = 0.0
    sus i normie = 0
    bestie i < len(predictions) {
        sus diff meal = abs_meal(predictions[i] - targets[i])
        sum_absolute_error = sum_absolute_error + diff
        i = i + 1
    }
    damn sum_absolute_error / len(predictions)
}

slay binary_crossentropy_loss(predictions []meal, targets []meal) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus loss meal = 0.0
    sus i normie = 0
    bestie i < len(predictions) {
        sus p meal = clamp_meal(predictions[i], EPSILON, 1.0 - EPSILON)
        sus t meal = targets[i]
        loss = loss - (t * ln_meal(p) + (1.0 - t) * ln_meal(1.0 - p))
        i = i + 1
    }
    damn loss / len(predictions)
}

fr fr === LINEAR REGRESSION ===

slay linear_model_weights_init(num_features normie) []meal {
    sus weights []meal = []
    sus i normie = 0
    bestie i < num_features {
        sus weight meal = random_gaussian() * 0.1
        weights = append(weights, weight)
        i = i + 1
    }
    damn weights
}

slay linear_model_predict_single(features []meal, weights []meal, bias meal) meal {
    lowkey len(features) != len(weights) {
        damn 0.0
    }
    
    sus prediction meal = bias
    sus i normie = 0
    bestie i < len(features) {
        prediction = prediction + features[i] * weights[i]
        i = i + 1
    }
    damn prediction
}

slay linear_model_predict_batch(feature_matrix []meal, num_samples normie, num_features normie, weights []meal, bias meal) []meal {
    sus predictions []meal = []
    sus i normie = 0
    bestie i < num_samples {
        sus features []meal = []
        sus j normie = 0
        bestie j < num_features {
            sus feature_idx normie = i * num_features + j
            features = append(features, feature_matrix[feature_idx])
            j = j + 1
        }
        sus prediction meal = linear_model_predict_single(features, weights, bias)
        predictions = append(predictions, prediction)
        i = i + 1
    }
    damn predictions
}

slay linear_model_train_step(feature_matrix []meal, targets []meal, weights []meal, bias meal, num_samples normie, num_features normie, learning_rate meal) ([]meal, meal) {
    fr fr Get current predictions
    sus predictions []meal = linear_model_predict_batch(feature_matrix, num_samples, num_features, weights, bias)
    
    fr fr Compute gradients
    sus bias_gradient meal = 0.0
    sus weight_gradients []meal = tensor_zeros_1d(num_features)
    
    sus i normie = 0
    bestie i < num_samples {
        sus error meal = predictions[i] - targets[i]
        bias_gradient = bias_gradient + error
        
        sus j normie = 0
        bestie j < num_features {
            sus feature_idx normie = i * num_features + j
            weight_gradients[j] = weight_gradients[j] + error * feature_matrix[feature_idx]
            j = j + 1
        }
        i = i + 1
    }
    
    fr fr Normalize gradients
    bias_gradient = bias_gradient / num_samples
    sus j normie = 0
    bestie j < num_features {
        weight_gradients[j] = weight_gradients[j] / num_samples
        j = j + 1
    }
    
    fr fr Update parameters
    sus new_bias meal = bias - learning_rate * bias_gradient
    sus new_weights []meal = []
    j = 0
    bestie j < num_features {
        sus new_weight meal = weights[j] - learning_rate * weight_gradients[j]
        new_weights = append(new_weights, new_weight)
        j = j + 1
    }
    
    damn (new_weights, new_bias)
}

fr fr === NEURAL NETWORK LAYER ===

slay layer_weights_init(input_size normie, output_size normie) []meal {
    fr fr Xavier initialization
    sus init_std meal = sqrt_meal(2.0 / (input_size + output_size))
    sus size normie = input_size * output_size
    sus weights []meal = []
    sus i normie = 0
    bestie i < size {
        sus weight meal = random_gaussian() * init_std
        weights = append(weights, weight)
        i = i + 1
    }
    damn weights
}

slay layer_biases_init(output_size normie) []meal {
    damn tensor_zeros_1d(output_size)
}

slay layer_forward_single(input []meal, weights []meal, biases []meal, input_size normie, output_size normie) []meal {
    sus output []meal = []
    sus j normie = 0
    bestie j < output_size {
        sus sum meal = biases[j]
        sus k normie = 0
        bestie k < input_size {
            sus weight_idx normie = k * output_size + j
            sum = sum + input[k] * weights[weight_idx]
            k = k + 1
        }
        output = append(output, sum)
        j = j + 1
    }
    damn output
}

slay layer_forward_batch(input_matrix []meal, weights []meal, biases []meal, batch_size normie, input_size normie, output_size normie) []meal {
    sus output_matrix []meal = []
    sus i normie = 0
    bestie i < batch_size {
        fr fr Extract input for this sample
        sus input []meal = []
        sus k normie = 0
        bestie k < input_size {
            sus input_idx normie = i * input_size + k
            input = append(input, input_matrix[input_idx])
            k = k + 1
        }
        
        fr fr Forward pass for this sample
        sus output []meal = layer_forward_single(input, weights, biases, input_size, output_size)
        
        fr fr Add to output matrix
        sus j normie = 0
        bestie j < output_size {
            output_matrix = append(output_matrix, output[j])
            j = j + 1
        }
        i = i + 1
    }
    damn output_matrix
}

fr fr === GRADIENT DESCENT OPTIMIZERS ===

slay sgd_update_weights(weights []meal, gradients []meal, learning_rate meal) []meal {
    sus new_weights []meal = []
    sus i normie = 0
    bestie i < len(weights) {
        sus new_weight meal = weights[i] - learning_rate * gradients[i]
        new_weights = append(new_weights, new_weight)
        i = i + 1
    }
    damn new_weights
}

slay momentum_update_weights(weights []meal, gradients []meal, momentum_weights []meal, learning_rate meal, momentum meal) ([]meal, []meal) {
    sus new_weights []meal = []
    sus new_momentum []meal = []
    sus i normie = 0
    bestie i < len(weights) {
        sus momentum_val meal = momentum * momentum_weights[i] + learning_rate * gradients[i]
        sus new_weight meal = weights[i] - momentum_val
        new_weights = append(new_weights, new_weight)
        new_momentum = append(new_momentum, momentum_val)
        i = i + 1
    }
    damn (new_weights, new_momentum)
}

fr fr === K-MEANS CLUSTERING ===

slay kmeans_init_centroids(data []meal, num_samples normie, num_features normie, k normie) []meal {
    sus centroids []meal = []
    sus i normie = 0
    bestie i < k {
        sus random_sample normie = random_range(0, num_samples)
        sus j normie = 0
        bestie j < num_features {
            sus data_idx normie = random_sample * num_features + j
            centroids = append(centroids, data[data_idx])
            j = j + 1
        }
        i = i + 1
    }
    damn centroids
}

slay kmeans_distance_squared(point1 []meal, point2 []meal) meal {
    lowkey len(point1) != len(point2) {
        damn INFINITY
    }
    
    sus distance meal = 0.0
    sus i normie = 0
    bestie i < len(point1) {
        sus diff meal = point1[i] - point2[i]
        distance = distance + diff * diff
        i = i + 1
    }
    damn distance
}

slay kmeans_assign_clusters(data []meal, centroids []meal, num_samples normie, num_features normie, k normie) []normie {
    sus assignments []normie = []
    sus i normie = 0
    bestie i < num_samples {
        sus min_distance meal = INFINITY
        sus best_cluster normie = 0
        
        fr fr Extract current point
        sus point []meal = []
        sus l normie = 0
        bestie l < num_features {
            sus data_idx normie = i * num_features + l
            point = append(point, data[data_idx])
            l = l + 1
        }
        
        fr fr Find closest centroid
        sus j normie = 0
        bestie j < k {
            fr fr Extract centroid
            sus centroid []meal = []
            l = 0
            bestie l < num_features {
                sus centroid_idx normie = j * num_features + l
                centroid = append(centroid, centroids[centroid_idx])
                l = l + 1
            }
            
            sus distance meal = kmeans_distance_squared(point, centroid)
            
            lowkey distance < min_distance {
                min_distance = distance
                best_cluster = j
            }
            j = j + 1
        }
        
        assignments = append(assignments, best_cluster)
        i = i + 1
    }
    damn assignments
}

slay kmeans_update_centroids(data []meal, assignments []normie, num_samples normie, num_features normie, k normie) []meal {
    sus centroids []meal = tensor_zeros_1d(k * num_features)
    sus cluster_counts []normie = tensor_fill(k, 0)
    
    fr fr Sum points in each cluster
    sus i normie = 0
    bestie i < num_samples {
        sus cluster normie = assignments[i]
        cluster_counts[cluster] = cluster_counts[cluster] + 1
        
        sus j normie = 0
        bestie j < num_features {
            sus data_idx normie = i * num_features + j
            sus centroid_idx normie = cluster * num_features + j
            centroids[centroid_idx] = centroids[centroid_idx] + data[data_idx]
            j = j + 1
        }
        i = i + 1
    }
    
    fr fr Average to get centroids
    sus j normie = 0
    bestie j < k {
        lowkey cluster_counts[j] > 0 {
            sus l normie = 0
            bestie l < num_features {
                sus centroid_idx normie = j * num_features + l
                centroids[centroid_idx] = centroids[centroid_idx] / cluster_counts[j]
                l = l + 1
            }
        }
        j = j + 1
    }
    
    damn centroids
}

slay kmeans_cluster(data []meal, num_samples normie, num_features normie, k normie, max_iterations normie) ([]meal, []normie) {
    sus centroids []meal = kmeans_init_centroids(data, num_samples, num_features, k)
    sus assignments []normie = []
    
    sus iteration normie = 0
    bestie iteration < max_iterations {
        sus new_assignments []normie = kmeans_assign_clusters(data, centroids, num_samples, num_features, k)
        
        fr fr Check for convergence
        lowkey len(assignments) == len(new_assignments) {
            sus converged lit = based
            sus i normie = 0
            bestie i < len(assignments) {
                lowkey assignments[i] != new_assignments[i] {
                    converged = cringe
                    ghosted
                }
                i = i + 1
            }
            
            lowkey converged {
                assignments = new_assignments
                ghosted
            }
        }
        
        assignments = new_assignments
        centroids = kmeans_update_centroids(data, assignments, num_samples, num_features, k)
        iteration = iteration + 1
    }
    
    damn (centroids, assignments)
}

fr fr === EVALUATION METRICS ===

slay accuracy_score(predictions []meal, targets []meal, threshold meal) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus correct normie = 0
    sus i normie = 0
    bestie i < len(predictions) {
        sus predicted normie = 0
        lowkey predictions[i] > threshold {
            predicted = 1
        }
        sus actual normie = 0
        lowkey targets[i] > threshold {
            actual = 1
        }
        lowkey predicted == actual {
            correct = correct + 1
        }
        i = i + 1
    }
    
    damn correct / len(predictions)
}

slay r2_score(predictions []meal, targets []meal) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus mean_targets meal = tensor_mean_1d(targets)
    sus ss_res meal = 0.0
    sus ss_tot meal = 0.0
    
    sus i normie = 0
    bestie i < len(predictions) {
        sus residual meal = targets[i] - predictions[i]
        sus total meal = targets[i] - mean_targets
        ss_res = ss_res + residual * residual
        ss_tot = ss_tot + total * total
        i = i + 1
    }
    
    lowkey ss_tot == 0.0 {
        damn 1.0
    }
    
    damn 1.0 - (ss_res / ss_tot)
}

fr fr === DATA PREPROCESSING ===

slay train_test_split_indices(num_samples normie, test_ratio meal) ([]normie, []normie) {
    sus test_size normie = num_samples * test_ratio
    sus train_size normie = num_samples - test_size
    
    sus train_indices []normie = []
    sus test_indices []normie = []
    
    sus i normie = 0
    bestie i < train_size {
        train_indices = append(train_indices, i)
        i = i + 1
    }
    
    bestie i < num_samples {
        test_indices = append(test_indices, i)
        i = i + 1
    }
    
    damn (train_indices, test_indices)
}

slay extract_samples_by_indices(data []meal, indices []normie, num_features normie) []meal {
    sus result []meal = []
    sus i normie = 0
    bestie i < len(indices) {
        sus sample_idx normie = indices[i]
        sus j normie = 0
        bestie j < num_features {
            sus data_idx normie = sample_idx * num_features + j
            result = append(result, data[data_idx])
            j = j + 1
        }
        i = i + 1
    }
    damn result
}

slay extract_labels_by_indices(labels []meal, indices []normie) []meal {
    sus result []meal = []
    sus i normie = 0
    bestie i < len(indices) {
        result = append(result, labels[indices[i]])
        i = i + 1
    }
    damn result
}

slay standardize_features_simple(features []meal, num_samples normie, num_features normie) ([]meal, []meal, []meal) {
    fr fr Compute means and stds for each feature
    sus means []meal = []
    sus stds []meal = []
    
    sus j normie = 0
    bestie j < num_features {
        fr fr Extract feature column
        sus feature_column []meal = []
        sus i normie = 0
        bestie i < num_samples {
            sus feature_idx normie = i * num_features + j
            feature_column = append(feature_column, features[feature_idx])
            i = i + 1
        }
        
        sus mean meal = tensor_mean_1d(feature_column)
        sus std meal = tensor_std_dev_1d(feature_column)
        lowkey std == 0.0 {
            std = 1.0
        }
        
        means = append(means, mean)
        stds = append(stds, std)
        j = j + 1
    }
    
    fr fr Standardize features
    sus standardized []meal = []
    sus i normie = 0
    bestie i < num_samples {
        j = 0
        bestie j < num_features {
            sus feature_idx normie = i * num_features + j
            sus standardized_val meal = (features[feature_idx] - means[j]) / stds[j]
            standardized = append(standardized, standardized_val)
            j = j + 1
        }
        i = i + 1
    }
    
    damn (standardized, means, stds)
}

fr fr === UTILITY FUNCTIONS ===

slay shuffle_data_indices(num_samples normie) []normie {
    sus indices []normie = []
    sus i normie = 0
    bestie i < num_samples {
        indices = append(indices, i)
        i = i + 1
    }
    
    fr fr Fisher-Yates shuffle
    i = num_samples - 1
    bestie i > 0 {
        sus j normie = random_range(0, i + 1)
        sus temp normie = indices[i]
        indices[i] = indices[j]
        indices[j] = temp
        i = i - 1
    }
    
    damn indices
}

slay print_training_progress(epoch normie, loss meal, accuracy meal) cringe {
    vibez.spill("Epoch ", epoch, ": Loss = ", loss, ", Accuracy = ", accuracy * 100.0, "%")
}

slay create_batches(num_samples normie, batch_size normie) [][]normie {
    sus batches [][]normie = []
    sus i normie = 0
    bestie i < num_samples {
        sus batch []normie = []
        sus end normie = i + batch_size
        lowkey end > num_samples {
            end = num_samples
        }
        
        sus j normie = i
        bestie j < end {
            batch = append(batch, j)
            j = j + 1
        }
        
        lowkey len(batch) > 0 {
            batches = append(batches, batch)
        }
        i = end
    }
    damn batches
}

fr fr === DEMONSTRATION FUNCTIONS ===

slay demo_linear_regression(features []meal, targets []meal, num_samples normie, num_features normie, epochs normie, learning_rate meal) cringe {
    vibez.spill("=== Linear Regression Demo ===")
    
    fr fr Initialize model
    sus weights []meal = linear_model_weights_init(num_features)
    sus bias meal = 0.0
    
    fr fr Training loop
    sus epoch normie = 0
    bestie epoch < epochs {
        sus updated_weights []meal
        sus updated_bias meal
        (updated_weights, updated_bias) = linear_model_train_step(features, targets, weights, bias, num_samples, num_features, learning_rate)
        weights = updated_weights
        bias = updated_bias
        
        lowkey epoch % (epochs / 10) == 0 {
            sus predictions []meal = linear_model_predict_batch(features, num_samples, num_features, weights, bias)
            sus loss meal = mse_loss(predictions, targets)
            sus r2 meal = r2_score(predictions, targets)
            print_training_progress(epoch, loss, r2)
        }
        epoch = epoch + 1
    }
    
    fr fr Final evaluation
    sus final_predictions []meal = linear_model_predict_batch(features, num_samples, num_features, weights, bias)
    sus final_loss meal = mse_loss(final_predictions, targets)
    sus final_r2 meal = r2_score(final_predictions, targets)
    
    vibez.spill("Final Loss: ", final_loss)
    vibez.spill("Final R²: ", final_r2)
}

slay demo_kmeans_clustering(data []meal, num_samples normie, num_features normie, k normie) cringe {
    vibez.spill("=== K-Means Clustering Demo ===")
    
    sus centroids []meal
    sus assignments []normie
    (centroids, assignments) = kmeans_cluster(data, num_samples, num_features, k, 100)
    
    vibez.spill("Cluster assignments:")
    sus i normie = 0
    bestie i < num_samples {
        vibez.spill("Sample ", i, ": Cluster ", assignments[i])
        i = i + 1
    }
}
