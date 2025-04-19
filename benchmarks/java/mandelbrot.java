// Mandelbrot set calculation benchmark

public class ClassName {
    // Size constants
    private static final int WIDTH = 800;
    private static final int HEIGHT = 800;
    private static final int MAX_ITERATIONS = 100;
    
    // Calculate the Mandelbrot set
    private static int[][] calculateMandelbrot(int maxIterations) {
        int[][] result = new int[HEIGHT][WIDTH];
        
        for (int y = 0; y < HEIGHT; y++) {
            for (int x = 0; x < WIDTH; x++) {
                double cx = (x - WIDTH / 2.0) * 4.0 / WIDTH;
                double cy = (y - HEIGHT / 2.0) * 4.0 / HEIGHT;
                
                double zx = 0.0;
                double zy = 0.0;
                int iteration = 0;
                
                while (zx*zx + zy*zy <= 4.0 && iteration < maxIterations) {
                    double temp = zx*zx - zy*zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = temp;
                    iteration++;
                }
                
                result[y][x] = iteration;
            }
        }
        
        return result;
    }
    
    // Count non-black pixels in the result
    private static int countNonBlack(int[][] result, int maxIterations) {
        int count = 0;
        
        for (int y = 0; y < HEIGHT; y++) {
            for (int x = 0; x < WIDTH; x++) {
                if (result[y][x] < maxIterations) {
                    count++;
                }
            }
        }
        
        return count;
    }
    
    public static void main(String[] args) {
        long startTime = System.currentTimeMillis();
        
        int[][] result = calculateMandelbrot(MAX_ITERATIONS);
        int count = countNonBlack(result, MAX_ITERATIONS);
        
        System.out.printf("Mandelbrot set calculation finished.\n");
        System.out.printf("Image size: %d x %d\n", WIDTH, HEIGHT);
        System.out.printf("Maximum iterations: %d\n", MAX_ITERATIONS);
        System.out.printf("Non-black pixels: %d\n", count);
        
        long elapsed = System.currentTimeMillis() - startTime;
        System.out.printf("Time taken: %d ms\n", elapsed);
        
        // Get memory stats
        long memoryUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1024;
        System.out.printf("Memory used: %d KB\n", memoryUsed);
    }
}