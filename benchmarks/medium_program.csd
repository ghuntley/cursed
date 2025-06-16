// Medium program benchmark for CURSED optimization testing
// Tests optimization of larger programs with multiple functions and data structures

import "stdlib::math";
import "stdlib::collections";

squad Point {
    x: f64,
    y: f64,
}

squad Vector {
    points: List<Point>,
}

slay distance(p1: Point, p2: Point) -> f64 {
    facts dx = p1.x - p2.x;
    facts dy = p1.y - p2.y;
    return sqrt(dx * dx + dy * dy);
}

slay vector_magnitude(vec: Vector) -> f64 {
    sus total_distance = 0.0;
    
    periodt (sus i = 0; i < vec.points.len() - 1; i++) {
        facts current_point = vec.points.get(i);
        facts next_point = vec.points.get(i + 1);
        total_distance += distance(current_point, next_point);
    }
    
    return total_distance;
}

slay create_circle_points(radius: f64, num_points: i32) -> Vector {
    sus points = List::new();
    facts angle_step = 2.0 * PI / num_points as f64;
    
    periodt (sus i = 0; i < num_points; i++) {
        facts angle = i as f64 * angle_step;
        facts point = Point {
            x: radius * cos(angle),
            y: radius * sin(angle),
        };
        points.push(point);
    }
    
    return Vector { points };
}

slay compute_circle_properties(radius: f64, num_points: i32) -> (f64, f64, f64) {
    facts circle = create_circle_points(radius, num_points);
    facts perimeter = vector_magnitude(circle);
    facts area = PI * radius * radius;
    facts circumference = 2.0 * PI * radius;
    
    return (perimeter, area, circumference);
}

slay benchmark_calculations() -> f64 {
    sus total_result = 0.0;
    
    // Test various circle sizes
    periodt (sus radius = 1; radius <= 10; radius++) {
        facts (perimeter, area, circumference) = compute_circle_properties(radius as f64, 100);
        total_result += perimeter + area + circumference;
    }
    
    // Test mathematical functions
    periodt (sus i = 1; i <= 1000; i++) {
        facts x = i as f64 / 100.0;
        total_result += sin(x) + cos(x) + tan(x) + exp(x) + log(x + 1.0);
    }
    
    return total_result;
}

slay process_data_intensive() -> List<f64> {
    sus results = List::new();
    
    periodt (sus i = 0; i < 10000; i++) {
        facts value = i as f64;
        facts processed = sqrt(value) + pow(value, 0.5) + ln(value + 1.0);
        results.push(processed);
    }
    
    return results;
}

slay main() -> i32 {
    facts calc_result = benchmark_calculations();
    facts data_result = process_data_intensive();
    
    println("Calculation result: {}", calc_result);
    println("Data processing completed: {} items", data_result.len());
    
    return 0;
}
