! Mandelbrot set calculation benchmark

program mandelbrot
  use iso_fortran_env, only: dp => real64, stdout => output_unit
  implicit none

  ! Size constants
  integer, parameter :: WIDTH = 800
  integer, parameter :: HEIGHT = 800
  integer, parameter :: MAX_ITERATIONS = 100

  integer, dimension(HEIGHT, WIDTH) :: result
  integer :: count, start_time, end_time, count_rate

  call system_clock(start_time, count_rate)

  ! Calculate the Mandelbrot set
  call calculate_mandelbrot(result, MAX_ITERATIONS)
  count = count_non_black(result, MAX_ITERATIONS)

  write(stdout, '(A)') "Mandelbrot set calculation finished."
  write(stdout, '(A,I0,A,I0)') "Image size: ", WIDTH, " x ", HEIGHT
  write(stdout, '(A,I0)') "Maximum iterations: ", MAX_ITERATIONS
  write(stdout, '(A,I0)') "Non-black pixels: ", count

  call system_clock(end_time)
  write(stdout, '(A,I0,A)') "Time taken: ", (end_time - start_time) * 1000 / count_rate, " ms"

contains

  ! Calculate the Mandelbrot set
  subroutine calculate_mandelbrot(result, max_iterations)
    integer, intent(out) :: result(HEIGHT, WIDTH)
    integer, intent(in) :: max_iterations
    integer :: x, y, iteration
    real(dp) :: cx, cy, zx, zy, temp

    do y = 1, HEIGHT
      do x = 1, WIDTH
        cx = (real(x, dp) - real(WIDTH, dp)/2.0_dp) * 4.0_dp / real(WIDTH, dp)
        cy = (real(y, dp) - real(HEIGHT, dp)/2.0_dp) * 4.0_dp / real(HEIGHT, dp)

        zx = 0.0_dp
        zy = 0.0_dp
        iteration = 0

        do while (zx*zx + zy*zy <= 4.0_dp .and. iteration < max_iterations)
          temp = zx*zx - zy*zy + cx
          zy = 2.0_dp*zx*zy + cy
          zx = temp
          iteration = iteration + 1
        end do

        result(y, x) = iteration
      end do
    end do
  end subroutine calculate_mandelbrot

  ! Count non-black pixels in the result
  function count_non_black(result, max_iterations) result(count)
    integer, intent(in) :: result(HEIGHT, WIDTH)
    integer, intent(in) :: max_iterations
    integer :: count, x, y

    count = 0
    do y = 1, HEIGHT
      do x = 1, WIDTH
        if (result(y, x) < max_iterations) then
          count = count + 1
        end if
      end do
    end do
  end function count_non_black

end program mandelbrot