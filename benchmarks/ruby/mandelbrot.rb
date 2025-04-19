# Mandelbrot set calculation benchmark for Ruby

# Constants
WIDTH = 800
HEIGHT = 800
MAX_ITERATIONS = 100

# Calculate Mandelbrot set for a single point
def mandelbrot_point(cx, cy, max_iter)
  zx = 0.0
  zy = 0.0
  iteration = 0
  
  while zx*zx + zy*zy <= 4.0 && iteration < max_iter
    temp = zx*zx - zy*zy + cx
    zy = 2.0*zx*zy + cy
    zx = temp
    iteration += 1
  end
  
  iteration
end

# Calculate entire Mandelbrot set
def calculate_mandelbrot
  result = Array.new(HEIGHT) { Array.new(WIDTH, 0) }
  
  (0...HEIGHT).each do |y|
    (0...WIDTH).each do |x|
      cx = (x.to_f - WIDTH/2.0) * 4.0 / WIDTH
      cy = (y.to_f - HEIGHT/2.0) * 4.0 / HEIGHT
      result[y][x] = mandelbrot_point(cx, cy, MAX_ITERATIONS)
    end
  end
  
  result
end

# Count non-black pixels
def count_non_black(result)
  count = 0
  
  (0...HEIGHT).each do |y|
    (0...WIDTH).each do |x|
      count += 1 if result[y][x] < MAX_ITERATIONS
    end
  end
  
  count
end

# Main function
def main
  start_time = Time.now
  
  result = calculate_mandelbrot
  count = count_non_black(result)
  
  puts "Mandelbrot set calculation finished."
  puts "Image size: #{WIDTH} x #{HEIGHT}"
  puts "Maximum iterations: #{MAX_ITERATIONS}"
  puts "Non-black pixels: #{count}"
  
  elapsed = Time.now - start_time
  puts "Time taken: #{elapsed * 1000} ms"
end

main