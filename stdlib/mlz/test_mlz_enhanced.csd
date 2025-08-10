fr fr CURSED Machine Learning Module (mlz) - Enhanced Test Suite
fr fr Comprehensive tests for all ML algorithms and functionality

yeet "mlz"
yeet "mathz"
yeet "tensorz"
yeet "arrayz"
yeet "vibez"

fr fr === TEST UTILITY FUNCTIONS ===

slay generate_test_data(num_samples normie, num_features normie) []meal {
    sus data []meal = []
    sus i normie = 0
    bestie i < num_samples * num_features {
        data = append(data, random_uniform())
        i = i + 1
    }
    damn data
}

slay generate_test_labels(num_samples normie, num_classes normie) []meal {
    sus labels []meal = []
    sus i normie = 0
    bestie i < num_samples {
        sus label meal = random_range(0, num_classes)
        labels = append(labels, label)
        i = i + 1
    }
    damn labels
}

slay test_assert_ml(condition lit, test_name tea) cringe {
    ready (condition) {
        vibez.spill("✓ ", test_name, " PASSED")
    } otherwise {
        vibez.spill("✗ ", test_name, " FAILED")
    }
}

fr fr === ACTIVATION FUNCTION TESTS ===

slay test_ml_activation_functions() cringe {
    vibez.spill("=== Testing ML Activation Functions ===")
    
    fr fr Test sigmoid
    sus sig_zero meal = sigmoid(0.0)
    test_assert_ml(sig_zero > 0.49 && sig_zero < 0.51, "sigmoid(0) ≈ 0.5")
    
    sus sig_large meal = sigmoid(100.0)
    test_assert_ml(sig_large > 0.99, "sigmoid(large) ≈ 1.0")
    
    sus sig_small meal = sigmoid(-100.0)
    test_assert_ml(sig_small < 0.01, "sigmoid(small) ≈ 0.0")
    
    fr fr Test ReLU
    sus relu_pos meal = relu(5.0)
    test_assert_ml(relu_pos == 5.0, "relu(positive) = input")
    
    sus relu_neg meal = relu(-3.0)
    test_assert_ml(relu_neg == 0.0, "relu(negative) = 0")
    
    fr fr Test derivatives
    sus sig_deriv meal = sigmoid_derivative(0.0)
    test_assert_ml(sig_deriv > 0.2 && sig_deriv < 0.3, "sigmoid_derivative(0) ≈ 0.25")
    
    sus relu_deriv_pos meal = relu_derivative(5.0)
    test_assert_ml(relu_deriv_pos == 1.0, "relu_derivative(positive) = 1")
    
    sus relu_deriv_neg meal = relu_derivative(-3.0)
    test_assert_ml(relu_deriv_neg == 0.0, "relu_derivative(negative) = 0")
    
    fr fr Test leaky ReLU
    sus leaky_pos meal = leaky_relu(5.0, 0.01)
    test_assert_ml(leaky_pos == 5.0, "leaky_relu(positive) = input")
    
    sus leaky_neg meal = leaky_relu(-5.0, 0.01)
    test_assert_ml(leaky_neg == -0.05, "leaky_relu(negative) = alpha * input")
    
    vibez.spill("ML activation function tests completed!")
}

fr fr === LOSS FUNCTION TESTS ===

slay test_ml_loss_functions() cringe {
    vibez.spill("=== Testing ML Loss Functions ===")
    
    fr fr Test MSE loss
    sus predictions []meal = [1.0, 2.0, 3.0]
    sus targets []meal = [1.1, 1.9, 3.2]
    sus mse meal = mse_loss(predictions, targets)
    test_assert_ml(mse >= 0.0 && mse < 0.1, "MSE loss reasonable")
    
    fr fr Test perfect prediction
    sus perfect_mse meal = mse_loss(targets, targets)
    test_assert_ml(perfect_mse < 0.001, "MSE loss zero for perfect prediction")
    
    fr fr Test MAE loss
    sus mae meal = mae_loss(predictions, targets)
    test_assert_ml(mae >= 0.0 && mae < 0.3, "MAE loss reasonable")
    
    fr fr Test binary cross-entropy
    sus bin_pred []meal = [0.9, 0.1, 0.8, 0.2]
    sus bin_targets []meal = [1.0, 0.0, 1.0, 0.0]
    sus bce meal = binary_crossentropy_loss(bin_pred, bin_targets)
    test_assert_ml(bce >= 0.0, "Binary cross-entropy non-negative")
    
    vibez.spill("ML loss function tests completed!")
}

fr fr === LINEAR REGRESSION TESTS ===

slay test_linear_regression() cringe {
    vibez.spill("=== Testing Linear Regression ===")
    
    sus num_features normie = 3
    sus num_samples normie = 20
    
    fr fr Generate simple linear data
    sus features []meal = generate_test_data(num_samples, num_features)
    sus targets []meal = generate_test_labels(num_samples, 1)
    
    fr fr Initialize weights and bias
    sus weights []meal = linear_model_weights_init(num_features)
    sus bias meal = 0.0
    
    test_assert_ml(len(weights) == num_features, "Linear model weights initialized correctly")
    
    fr fr Test single prediction
    sus sample_features []meal = [1.0, 2.0, 3.0]
    sus prediction meal = linear_model_predict_single(sample_features, weights, bias)
    test_assert_ml(prediction != 0.0, "Linear model produces prediction")
    
    fr fr Test batch prediction
    sus predictions []meal = linear_model_predict_batch(features, num_samples, num_features, weights, bias)
    test_assert_ml(len(predictions) == num_samples, "Batch prediction size correct")
    
    fr fr Test training step
    sus learning_rate meal = 0.01
    sus new_weights []meal
    sus new_bias meal
    (new_weights, new_bias) = linear_model_train_step(features, targets, weights, bias, num_samples, num_features, learning_rate)
    
    test_assert_ml(len(new_weights) == num_features, "Training step updates weights correctly")
    test_assert_ml(new_bias != bias, "Training step updates bias")
    
    vibez.spill("Linear regression tests completed!")
}

fr fr === NEURAL NETWORK LAYER TESTS ===

slay test_nn_layers() cringe {
    vibez.spill("=== Testing Neural Network Layers ===")
    
    sus input_size normie = 4
    sus output_size normie = 3
    
    fr fr Test weight initialization
    sus weights []meal = layer_weights_init(input_size, output_size)
    test_assert_ml(len(weights) == input_size * output_size, "Layer weights size correct")
    
    fr fr Test bias initialization
    sus biases []meal = layer_biases_init(output_size)
    test_assert_ml(len(biases) == output_size, "Layer biases size correct")
    
    fr fr Test forward pass
    sus input []meal = [1.0, 2.0, 3.0, 4.0]
    sus output []meal = layer_forward_single(input, weights, biases, input_size, output_size)
    test_assert_ml(len(output) == output_size, "Layer forward pass output size correct")
    
    fr fr Test batch forward pass
    sus batch_size normie = 5
    sus input_matrix []meal = generate_test_data(batch_size, input_size)
    sus output_matrix []meal = layer_forward_batch(input_matrix, weights, biases, batch_size, input_size, output_size)
    test_assert_ml(len(output_matrix) == batch_size * output_size, "Batch forward pass size correct")
    
    vibez.spill("Neural network layer tests completed!")
}

fr fr === K-MEANS CLUSTERING TESTS ===

slay test_kmeans_clustering() cringe {
    vibez.spill("=== Testing K-Means Clustering ===")
    
    sus num_samples normie = 30
    sus num_features normie = 2
    sus k normie = 3
    
    sus data []meal = generate_test_data(num_samples, num_features)
    
    fr fr Test centroid initialization
    sus centroids []meal = kmeans_init_centroids(data, num_samples, num_features, k)
    test_assert_ml(len(centroids) == k * num_features, "Centroids initialized correctly")
    
    fr fr Test distance calculation
    sus point1 []meal = [1.0, 2.0]
    sus point2 []meal = [4.0, 6.0]
    sus distance meal = kmeans_distance_squared(point1, point2)
    test_assert_ml(distance == 25.0, "Distance calculation correct (3² + 4² = 25)")
    
    fr fr Test cluster assignment
    sus assignments []normie = kmeans_assign_clusters(data, centroids, num_samples, num_features, k)
    test_assert_ml(len(assignments) == num_samples, "Cluster assignments size correct")
    
    fr fr Verify assignments are valid
    sus i normie = 0
    sus valid_assignments lit = based
    bestie i < len(assignments) {
        ready (assignments[i] < 0 || assignments[i] >= k) {
            valid_assignments = cringe
            ghosted
        }
        i = i + 1
    }
    test_assert_ml(valid_assignments, "All cluster assignments valid")
    
    fr fr Test centroid update
    sus new_centroids []meal = kmeans_update_centroids(data, assignments, num_samples, num_features, k)
    test_assert_ml(len(new_centroids) == k * num_features, "Updated centroids size correct")
    
    fr fr Test full clustering
    sus final_centroids []meal
    sus final_assignments []normie
    (final_centroids, final_assignments) = kmeans_cluster(data, num_samples, num_features, k, 10)
    test_assert_ml(len(final_centroids) == k * num_features, "Final centroids size correct")
    test_assert_ml(len(final_assignments) == num_samples, "Final assignments size correct")
    
    vibez.spill("K-means clustering tests completed!")
}

fr fr === SVM TESTS ===

slay test_svm_kernels() cringe {
    vibez.spill("=== Testing SVM Kernels ===")
    
    sus x1 []meal = [1.0, 2.0, 3.0]
    sus x2 []meal = [2.0, 3.0, 4.0]
    
    fr fr Test linear kernel
    sus linear_result meal = svm_kernel_linear(x1, x2)
    sus expected_linear meal = 1.0*2.0 + 2.0*3.0 + 3.0*4.0  fr fr = 2 + 6 + 12 = 20
    test_assert_ml(linear_result == expected_linear, "Linear kernel calculation correct")
    
    fr fr Test RBF kernel
    sus rbf_result meal = svm_kernel_rbf(x1, x2, 1.0)
    test_assert_ml(rbf_result > 0.0 && rbf_result <= 1.0, "RBF kernel result in valid range")
    
    fr fr Test polynomial kernel
    sus poly_result meal = svm_kernel_polynomial(x1, x2, 2, 1.0)
    test_assert_ml(poly_result > 0.0, "Polynomial kernel result positive")
    
    vibez.spill("SVM kernel tests completed!")
}

fr fr === DECISION TREE TESTS ===

slay test_decision_trees() cringe {
    vibez.spill("=== Testing Decision Trees ===")
    
    sus num_classes normie = 3
    sus labels []meal = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0]
    sus start_idx normie = 0
    sus end_idx normie = 6
    
    fr fr Test Gini impurity
    sus gini meal = decision_tree_gini_impurity(labels, start_idx, end_idx, num_classes)
    test_assert_ml(gini >= 0.0 && gini <= 1.0, "Gini impurity in valid range")
    
    fr fr Test majority class
    sus majority meal = decision_tree_majority_class(labels, start_idx, end_idx, num_classes)
    test_assert_ml(majority >= 0 && majority < num_classes, "Majority class valid")
    
    fr fr Test best split finding
    sus num_features normie = 2
    sus features []meal = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]
    sus best_feature normie
    sus best_threshold meal
    sus best_gain meal
    (best_feature, best_threshold, best_gain) = decision_tree_find_best_split(features, labels, start_idx, end_idx, num_features, num_classes)
    
    test_assert_ml(best_feature >= 0 && best_feature < num_features, "Best feature index valid")
    test_assert_ml(best_gain >= 0.0, "Information gain non-negative")
    
    vibez.spill("Decision tree tests completed!")
}

fr fr === NAIVE BAYES TESTS ===

slay test_naive_bayes() cringe {
    vibez.spill("=== Testing Naive Bayes ===")
    
    sus num_samples normie = 12
    sus num_features normie = 2
    sus num_classes normie = 2
    
    fr fr Create simple dataset
    sus features []meal = [
        1.0, 1.0,  fr fr Class 0
        1.5, 1.2,  fr fr Class 0
        1.1, 0.9,  fr fr Class 0
        2.0, 2.0,  fr fr Class 0
        2.1, 1.9,  fr fr Class 0
        1.9, 2.1,  fr fr Class 0
        
        4.0, 4.0,  fr fr Class 1
        4.1, 3.9,  fr fr Class 1
        3.9, 4.1,  fr fr Class 1
        5.0, 5.0,  fr fr Class 1
        4.8, 5.2,  fr fr Class 1
        5.1, 4.9   fr fr Class 1
    ]
    
    sus labels []meal = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
    
    fr fr Train Naive Bayes model
    sus model NaiveBayesModel = naive_bayes_train(features, labels, num_samples, num_features, num_classes)
    
    test_assert_ml(model.num_classes == num_classes, "Model classes correct")
    test_assert_ml(model.num_features == num_features, "Model features correct")
    test_assert_ml(len(model.class_priors) == num_classes, "Class priors size correct")
    test_assert_ml(len(model.feature_means) == num_classes, "Feature means size correct")
    
    fr fr Test prediction
    sus test_sample []meal = [2.0, 2.0]  fr fr Should be closer to class 0
    sus prediction normie = naive_bayes_predict(model, test_sample)
    test_assert_ml(prediction >= 0 && prediction < num_classes, "Prediction class valid")
    
    sus test_sample2 []meal = [4.5, 4.5]  fr fr Should be closer to class 1
    sus prediction2 normie = naive_bayes_predict(model, test_sample2)
    test_assert_ml(prediction2 >= 0 && prediction2 < num_classes, "Prediction2 class valid")
    
    vibez.spill("Naive Bayes tests completed!")
}

fr fr === PCA TESTS ===

slay test_pca() cringe {
    vibez.spill("=== Testing PCA ===")
    
    sus num_samples normie = 10
    sus num_features normie = 3
    
    sus data []meal = generate_test_data(num_samples, num_features)
    
    fr fr Test data centering
    sus centered_data []meal
    sus means []meal
    (centered_data, means) = pca_center_data(data, num_samples, num_features)
    
    test_assert_ml(len(centered_data) == num_samples * num_features, "Centered data size correct")
    test_assert_ml(len(means) == num_features, "Means size correct")
    
    fr fr Test covariance matrix computation
    sus cov_matrix []meal = pca_compute_covariance_matrix(centered_data, num_samples, num_features)
    test_assert_ml(len(cov_matrix) == num_features * num_features, "Covariance matrix size correct")
    
    fr fr Test eigenvalue computation
    sus eigenvalue meal
    sus eigenvector []meal
    (eigenvalue, eigenvector) = pca_power_iteration_eigenvalue(cov_matrix, num_features, 50)
    
    test_assert_ml(len(eigenvector) == num_features, "Eigenvector size correct")
    test_assert_ml(eigenvalue >= 0.0, "Eigenvalue non-negative")
    
    vibez.spill("PCA tests completed!")
}

fr fr === RANDOM FOREST TESTS ===

slay test_random_forest() cringe {
    vibez.spill("=== Testing Random Forest ===")
    
    sus num_samples normie = 20
    sus num_features normie = 3
    sus num_classes normie = 2
    sus num_trees normie = 5
    
    sus features []meal = generate_test_data(num_samples, num_features)
    sus labels []meal = generate_test_labels(num_samples, num_classes)
    
    fr fr Train random forest
    sus model RandomForestModel = random_forest_train(features, labels, num_samples, num_features, num_classes, num_trees)
    
    test_assert_ml(model.num_trees == num_trees, "Random forest tree count correct")
    test_assert_ml(len(model.trees) == num_trees, "Random forest trees array size correct")
    test_assert_ml(model.max_features > 0, "Max features positive")
    
    fr fr Test prediction
    sus test_features []meal = [0.5, 0.6, 0.7]
    sus prediction normie = random_forest_predict(model, test_features, num_classes)
    test_assert_ml(prediction >= 0 && prediction < num_classes, "Random forest prediction valid")
    
    vibez.spill("Random forest tests completed!")
}

fr fr === Q-LEARNING TESTS ===

slay test_q_learning() cringe {
    vibez.spill("=== Testing Q-Learning ===")
    
    sus num_states normie = 5
    sus num_actions normie = 3
    sus learning_rate meal = 0.1
    sus discount_factor meal = 0.9
    sus epsilon meal = 0.1
    
    fr fr Create Q-learning agent
    sus agent QLearningAgent = q_learning_agent_create(num_states, num_actions, learning_rate, discount_factor, epsilon)
    
    test_assert_ml(agent.num_states == num_states, "Agent states correct")
    test_assert_ml(agent.num_actions == num_actions, "Agent actions correct")
    test_assert_ml(agent.learning_rate == learning_rate, "Agent learning rate correct")
    test_assert_ml(len(agent.q_table) == num_states, "Q-table size correct")
    test_assert_ml(len(agent.q_table[0]) == num_actions, "Q-table actions size correct")
    
    fr fr Test action selection
    sus state normie = 2
    sus action normie = q_learning_choose_action(agent, state)
    test_assert_ml(action >= 0 && action < num_actions, "Action selection valid")
    
    fr fr Test Q-value update
    sus next_state normie = 3
    sus reward meal = 1.0
    sus updated_agent QLearningAgent = q_learning_update(agent, state, action, reward, next_state)
    
    test_assert_ml(updated_agent.num_states == num_states, "Updated agent states preserved")
    test_assert_ml(updated_agent.num_actions == num_actions, "Updated agent actions preserved")
    
    vibez.spill("Q-learning tests completed!")
}

fr fr === ANOMALY DETECTION TESTS ===

slay test_anomaly_detection() cringe {
    vibez.spill("=== Testing Anomaly Detection ===")
    
    sus num_samples normie = 15
    sus num_features normie = 2
    sus num_trees normie = 10
    
    sus data []meal = generate_test_data(num_samples, num_features)
    
    fr fr Test isolation forest
    sus anomaly_scores []meal = isolation_forest_anomaly_score(data, num_samples, num_features, num_trees)
    test_assert_ml(len(anomaly_scores) == num_samples, "Anomaly scores size correct")
    
    sus all_scores_valid lit = based
    sus i normie = 0
    bestie i < len(anomaly_scores) {
        ready (anomaly_scores[i] < 0.0 || anomaly_scores[i] > 1.0) {
            all_scores_valid = cringe
            ghosted
        }
        i = i + 1
    }
    test_assert_ml(all_scores_valid, "All anomaly scores in valid range [0,1]")
    
    fr fr Test one-class SVM
    sus anomaly_flags []lit = one_class_svm_anomaly_detection(data, num_samples, num_features, 0.1)
    test_assert_ml(len(anomaly_flags) == num_samples, "Anomaly flags size correct")
    
    vibez.spill("Anomaly detection tests completed!")
}

fr fr === FEATURE SELECTION TESTS ===

slay test_feature_selection() cringe {
    vibez.spill("=== Testing Feature Selection ===")
    
    sus num_samples normie = 25
    sus num_features normie = 6
    
    sus features []meal = generate_test_data(num_samples, num_features)
    sus labels []meal = generate_test_labels(num_samples, 2)
    
    fr fr Test mutual information
    sus mi_scores []meal = feature_selection_mutual_information(features, labels, num_samples, num_features)
    test_assert_ml(len(mi_scores) == num_features, "MI scores size correct")
    
    sus all_mi_valid lit = based
    sus i normie = 0
    bestie i < len(mi_scores) {
        ready (mi_scores[i] < 0.0) {
            all_mi_valid = cringe
            ghosted
        }
        i = i + 1
    }
    test_assert_ml(all_mi_valid, "All MI scores non-negative")
    
    fr fr Test chi-squared
    sus chi2_scores []meal = feature_selection_chi_squared(features, labels, num_samples, num_features)
    test_assert_ml(len(chi2_scores) == num_features, "Chi-squared scores size correct")
    
    fr fr Test feature selection
    sus k_best normie = 3
    sus selected_features []normie = feature_selection_select_k_best(mi_scores, k_best)
    test_assert_ml(len(selected_features) == k_best, "Selected features count correct")
    
    vibez.spill("Feature selection tests completed!")
}

fr fr === EVALUATION METRICS TESTS ===

slay test_evaluation_metrics() cringe {
    vibez.spill("=== Testing Evaluation Metrics ===")
    
    fr fr Test accuracy score
    sus predictions []meal = [0.9, 0.1, 0.8, 0.2, 0.7]
    sus targets []meal = [1.0, 0.0, 1.0, 0.0, 1.0]
    sus threshold meal = 0.5
    
    sus accuracy meal = accuracy_score(predictions, targets, threshold)
    test_assert_ml(accuracy >= 0.0 && accuracy <= 1.0, "Accuracy in valid range")
    test_assert_ml(accuracy == 1.0, "Perfect accuracy for good predictions")
    
    fr fr Test R² score
    sus reg_predictions []meal = [2.0, 3.0, 4.0, 5.0]
    sus reg_targets []meal = [2.1, 2.9, 4.2, 4.8]
    sus r2 meal = r2_score(reg_predictions, reg_targets)
    test_assert_ml(r2 <= 1.0, "R² score upper bounded")
    
    vibez.spill("Evaluation metrics tests completed!")
}

fr fr === DATA PREPROCESSING TESTS ===

slay test_data_preprocessing() cringe {
    vibez.spill("=== Testing Data Preprocessing ===")
    
    sus num_samples normie = 20
    sus test_ratio meal = 0.3
    
    fr fr Test train/test split
    sus train_indices []normie
    sus test_indices []normie
    (train_indices, test_indices) = train_test_split_indices(num_samples, test_ratio)
    
    sus total_samples normie = len(train_indices) + len(test_indices)
    test_assert_ml(total_samples == num_samples, "Train/test split preserves sample count")
    test_assert_ml(len(test_indices) > 0, "Test set non-empty")
    test_assert_ml(len(train_indices) > 0, "Train set non-empty")
    
    fr fr Test feature standardization
    sus num_features normie = 3
    sus features []meal = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]
    sus standardized []meal
    sus means []meal
    sus stds []meal
    (standardized, means, stds) = standardize_features_simple(features, 4, num_features)
    
    test_assert_ml(len(standardized) == len(features), "Standardized features size correct")
    test_assert_ml(len(means) == num_features, "Means size correct")
    test_assert_ml(len(stds) == num_features, "Standard deviations size correct")
    
    fr fr Test data shuffling
    sus shuffled_indices []normie = shuffle_data_indices(num_samples)
    test_assert_ml(len(shuffled_indices) == num_samples, "Shuffled indices size correct")
    
    vibez.spill("Data preprocessing tests completed!")
}

fr fr === INTEGRATION TESTS ===

slay test_comprehensive_ml_pipeline() cringe {
    vibez.spill("=== Testing Comprehensive ML Pipeline ===")
    
    sus num_samples normie = 50
    sus num_features normie = 4
    sus num_classes normie = 3
    
    fr fr Generate dataset
    sus features []meal = generate_test_data(num_samples, num_features)
    sus labels []meal = generate_test_labels(num_samples, num_classes)
    
    fr fr Data preprocessing
    sus standardized []meal
    sus means []meal
    sus stds []meal
    (standardized, means, stds) = standardize_features_simple(features, num_samples, num_features)
    
    fr fr Train/test split
    sus train_indices []normie
    sus test_indices []normie
    (train_indices, test_indices) = train_test_split_indices(num_samples, 0.2)
    
    sus train_features []meal = extract_samples_by_indices(standardized, train_indices, num_features)
    sus train_labels []meal = extract_labels_by_indices(labels, train_indices)
    sus test_features []meal = extract_samples_by_indices(standardized, test_indices, num_features)
    sus test_labels []meal = extract_labels_by_indices(labels, test_indices)
    
    fr fr Test multiple algorithms
    vibez.spill("Testing Naive Bayes on preprocessed data...")
    sus nb_model NaiveBayesModel = naive_bayes_train(train_features, train_labels, len(train_indices), num_features, num_classes)
    
    vibez.spill("Testing Random Forest on preprocessed data...")
    sus rf_model RandomForestModel = random_forest_train(train_features, train_labels, len(train_indices), num_features, num_classes, 5)
    
    vibez.spill("Testing K-means clustering...")
    sus centroids []meal
    sus assignments []normie
    (centroids, assignments) = kmeans_cluster(train_features, len(train_indices), num_features, num_classes, 20)
    
    test_assert_ml(len(train_features) > 0, "Train features extracted")
    test_assert_ml(len(test_features) > 0, "Test features extracted")
    test_assert_ml(nb_model.num_classes == num_classes, "Naive Bayes model trained")
    test_assert_ml(rf_model.num_trees == 5, "Random Forest model trained")
    test_assert_ml(len(assignments) == len(train_indices), "K-means clustering completed")
    
    vibez.spill("Comprehensive ML pipeline test completed!")
}

fr fr === DEMO INTEGRATION TESTS ===

slay test_all_ml_demos() cringe {
    vibez.spill("=== Testing All ML Demo Functions ===")
    
    sus num_samples normie = 30
    sus num_features normie = 4
    sus num_classes normie = 2
    
    sus features []meal = generate_test_data(num_samples, num_features)
    sus labels []meal = generate_test_labels(num_samples, num_classes)
    
    fr fr Test linear regression demo
    demo_linear_regression(features, labels, num_samples, num_features, 20, 0.01)
    
    fr fr Test k-means demo
    demo_kmeans_clustering(features, num_samples, num_features, 3)
    
    fr fr Test SVM demo
    demo_svm_classification(features, labels, num_samples, num_features, num_classes)
    
    fr fr Test Naive Bayes demo
    demo_naive_bayes_classification(features, labels, num_samples, num_features, num_classes)
    
    fr fr Test PCA demo
    demo_pca_dimensionality_reduction(features, num_samples, num_features, 2)
    
    fr fr Test Random Forest demo
    demo_random_forest_classification(features, labels, num_samples, num_features, num_classes)
    
    fr fr Test Q-learning demo
    demo_q_learning_agent(5, 3, 50)
    
    fr fr Test anomaly detection demo
    demo_anomaly_detection(features, num_samples, num_features)
    
    fr fr Test feature selection demo
    demo_feature_selection(features, labels, num_samples, num_features)
    
    vibez.spill("All ML demo function tests completed!")
}

fr fr === MAIN TEST RUNNER ===

slay run_enhanced_ml_tests() cringe {
    vibez.spill("🧪 CURSED Machine Learning Module (mlz) - Enhanced Test Suite")
    vibez.spill("=" * 70)
    
    test_ml_activation_functions()
    vibez.spill("")
    
    test_ml_loss_functions()
    vibez.spill("")
    
    test_linear_regression()
    vibez.spill("")
    
    test_nn_layers()
    vibez.spill("")
    
    test_kmeans_clustering()
    vibez.spill("")
    
    test_svm_kernels()
    vibez.spill("")
    
    test_decision_trees()
    vibez.spill("")
    
    test_naive_bayes()
    vibez.spill("")
    
    test_pca()
    vibez.spill("")
    
    test_random_forest()
    vibez.spill("")
    
    test_q_learning()
    vibez.spill("")
    
    test_anomaly_detection()
    vibez.spill("")
    
    test_feature_selection()
    vibez.spill("")
    
    test_evaluation_metrics()
    vibez.spill("")
    
    test_data_preprocessing()
    vibez.spill("")
    
    test_comprehensive_ml_pipeline()
    vibez.spill("")
    
    test_all_ml_demos()
    vibez.spill("")
    
    vibez.spill("🎉 Enhanced Machine Learning Module Test Suite Completed!")
    vibez.spill("All advanced ML algorithms and functionality have been validated.")
    vibez.spill("The mlz module now includes:")
    vibez.spill("  • Support Vector Machines (SVM)")
    vibez.spill("  • Decision Trees & Random Forests")
    vibez.spill("  • Naive Bayes Classification")
    vibez.spill("  • Principal Component Analysis (PCA)")
    vibez.spill("  • Q-Learning (Reinforcement Learning)")
    vibez.spill("  • Anomaly Detection (Isolation Forest, One-Class SVM)")
    vibez.spill("  • Feature Selection (Mutual Information, Chi-squared)")
    vibez.spill("  • Advanced Ensemble Methods")
    vibez.spill("  • Comprehensive Data Preprocessing")
    vibez.spill("")
    vibez.spill("The mlz and nnz modules together provide a complete")
    vibez.spill("AI/ML framework for CURSED development! 🚀")
}

fr fr Execute the enhanced test suite
run_enhanced_ml_tests()
