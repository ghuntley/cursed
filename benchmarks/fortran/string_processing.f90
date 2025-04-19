! String processing benchmark

program string_processing
  use iso_fortran_env, only: stdout => output_unit
  implicit none

  character(len=62) :: chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
  character(len=:), allocatable :: small, medium, large
  integer :: result_length, start_time, end_time, count_rate
  integer, parameter :: seed = 42

  ! Seed the random number generator
  call random_seed(put=(/seed/))
  call system_clock(start_time, count_rate)

  ! Process strings of different sizes
  small = process_strings(10000, 10)   ! 10,000 strings of length 10
  medium = process_strings(1000, 100)  ! 1,000 strings of length 100
  large = process_strings(100, 1000)   ! 100 strings of length 1,000

  result_length = len(small) + len(medium) + len(large)
  write(stdout, '(A,I0)') "Processed string length: ", result_length

  call system_clock(end_time)
  write(stdout, '(A,I0,A)') "Time taken: ", (end_time - start_time) * 1000 / count_rate, " ms"

contains

  ! Process multiple strings
  function process_strings(count, size) result(result_str)
    integer, intent(in) :: count, size
    character(len=:), allocatable :: result_str
    character(len=:), allocatable :: str, processed
    integer :: i
    
    result_str = ""
    
    do i = 1, count
      str = create_random_string(size)
      processed = process_string(str)
      result_str = result_str // processed
    end do
  end function process_strings

  ! Create a random string of given size
  function create_random_string(size) result(str)
    integer, intent(in) :: size
    character(len=size) :: str
    integer :: i, idx
    real :: r
    
    do i = 1, size
      call random_number(r)
      idx = int(r * 62) + 1
      str(i:i) = chars(idx:idx)
    end do
  end function create_random_string

  ! Process a single string
  function process_string(input) result(output)
    character(len=*), intent(in) :: input
    character(len=:), allocatable :: output, reversed
    character(len=1) :: first, ch
    integer :: i, j, half_len, input_len, digit
    
    output = input
    input_len = len(input)
    
    ! Replace all vowels with their uppercase version
    do i = 1, input_len
      ch = output(i:i)
      select case (ch)
        case ('a')
          output(i:i) = 'A'
        case ('e')
          output(i:i) = 'E'
        case ('i')
          output(i:i) = 'I'
        case ('o')
          output(i:i) = 'O'
        case ('u')
          output(i:i) = 'U'
      end select
    end do
    
    ! Replace all digits with their doubled value
    do i = 1, input_len
      ch = output(i:i)
      select case (ch)
        case ('0')
          output(i:i) = '0'
        case ('1')
          output(i:i) = '2'
        case ('2')
          output(i:i) = '4'
        case ('3')
          output(i:i) = '6'
        case ('4')
          output(i:i) = '8'
        case ('5')
          output(i:i) = '0'  ! 5*2=10, take last digit
        case ('6')
          output(i:i) = '2'  ! 6*2=12, take last digit
        case ('7')
          output(i:i) = '4'  ! 7*2=14, take last digit
        case ('8')
          output(i:i) = '6'  ! 8*2=16, take last digit
        case ('9')
          output(i:i) = '8'  ! 9*2=18, take last digit
      end select
    end do
    
    ! Capitalize the first letter
    if (input_len > 0) then
      first = output(1:1)
      call uppercase(first)
      output(1:1) = first
    end if
    
    ! Reverse the string
    reversed = ""
    do i = input_len, 1, -1
      reversed = reversed // output(i:i)
    end do
    
    ! Take the first half of the reversed string
    half_len = len(reversed) / 2
    output = reversed(1:half_len)
  end function process_string

  ! Convert a character to uppercase
  subroutine uppercase(ch)
    character(len=1), intent(inout) :: ch
    integer :: i
    
    i = iachar(ch)
    if (i >= iachar('a') .and. i <= iachar('z')) then
      ch = achar(i - iachar('a') + iachar('A'))
    end if
  end subroutine uppercase

end program string_processing