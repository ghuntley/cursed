// Mandelbrot set calculation benchmark

using System;

class Mandelbrot
{
    // Size constants
    private const int WIDTH = 800;
    private const int HEIGHT = 800;
    private const int MAX_ITERATIONS = 100;
    
    // Calculate the Mandelbrot set
    private static int[,] CalculateMandelbrot(int maxIterations)
    {
        int[,] result = new int[HEIGHT, WIDTH];
        
        for (int y = 0; y < HEIGHT; y++)
        {
            for (int x = 0; x < WIDTH; x++)
            {
                double cx = (x - WIDTH / 2.0) * 4.0 / WIDTH;
                double cy = (y - HEIGHT / 2.0) * 4.0 / HEIGHT;
                
                double zx = 0.0;
                double zy = 0.0;
                int iteration = 0;
                
                while (zx*zx + zy*zy <= 4.0 && iteration < maxIterations)
                {
                    double temp = zx*zx - zy*zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = temp;
                    iteration++;
                }
                
                result[y, x] = iteration;
            }
        }
        
        return result;
    }
    
    // Count non-black pixels
    private static int CountNonBlack(int[,] result, int maxIterations)
    {
        int count = 0;
        
        for (int y = 0; y < HEIGHT; y++)
        {
            for (int x = 0; x < WIDTH; x++)
            {
                if (result[y, x] < maxIterations)
                {
                    count++;
                }
            }
        }
        
        return count;
    }
    
    public static void Main()
    {
        DateTime startTime = DateTime.Now;
        
        int[,] result = CalculateMandelbrot(MAX_ITERATIONS);
        int count = CountNonBlack(result, MAX_ITERATIONS);
        
        Console.WriteLine("Mandelbrot set calculation finished.");
        Console.WriteLine("Image size: {0} x {1}", WIDTH, HEIGHT);
        Console.WriteLine("Maximum iterations: {0}", MAX_ITERATIONS);
        Console.WriteLine("Non-black pixels: {0}", count);
        
        TimeSpan elapsed = DateTime.Now - startTime;
        Console.WriteLine("Time taken: {0} ms", elapsed.TotalMilliseconds);
        
        // Get memory stats
        long memoryUsed = GC.GetTotalMemory(true) / 1024;
        Console.WriteLine("Memory used: {0} KB", memoryUsed);
    }
}