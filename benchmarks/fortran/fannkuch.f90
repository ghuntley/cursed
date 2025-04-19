! Fannkuch redux benchmark

program fannkuch
  use iso_fortran_env, only: int32, stdout => output_unit
  implicit none

  integer(int32), parameter :: n = 10
  integer(int32) :: result, start_time, end_time, count_rate

  call system_clock(start_time, count_rate)

  result = fannkuch_redux(n)

  write(stdout, '(A,I0,A,I0)') "Fannkuch(", n, "): ", result

  call system_clock(end_time)
  write(stdout, '(A,I0,A)') "Time taken: ", (end_time - start_time) * 1000 / count_rate, " ms"

contains

  ! Reverse the first n elements of the array
  subroutine flip(p, n)
    integer(int32), intent(inout) :: p(:)
    integer(int32), intent(in) :: n
    integer(int32) :: i, temp

    do i = 1, n/2
      temp = p(i)
      p(i) = p(n-i+1)
      p(n-i+1) = temp
    end do
  end subroutine flip

  ! Count flips required to flip elements to get back to original order
  function fannkuch_redux(n) result(max_flips)
    integer(int32), intent(in) :: n
    integer(int32) :: max_flips
    integer(int32) :: p(n), perm(n), count(n)
    integer(int32) :: i, j, flips, k, first_j, perm_count, sign, checksum, temp
    logical :: done

    ! Initialize permutation
    do i = 1, n
      p(i) = i - 1
    end do

    perm_count = 0
    sign = 1
    max_flips = 0
    checksum = 0
    done = .false.

    do while (.not. done .and. perm_count < 10000)
      ! Copy permutation to perm
      do i = 1, n
        perm(i) = p(i) + 1
      end do

      first = p(1)
      if (first /= 0) then
        ! Count flips
        count = 0
        flips = 0

        do while (perm(1) /= 1)
          k = perm(1) - 1
          call flip(perm, k)
          flips = flips + 1
          perm(1) = k + 1
        end do

        if (flips > max_flips) then
          max_flips = flips
        end if

        checksum = checksum + sign * flips
      end if

      ! Generate next permutation
      sign = -sign
      j = 2
      do while (j <= n .and. p(j-1) >= p(j))
        j = j + 1
      end do

      if (j > n) then
        done = .true.
        cycle
      end if

      perm_count = perm_count + 1

      first_j = p(j)
      do i = 1, j-1
        if (mod(i, 2) == 1) then
          temp = p(i)
          p(i) = p(j-i+1)
          p(j-i+1) = temp
        else
          temp = p(i)
          p(i) = p(j-i)
          p(j-i) = temp
        end if
      end do

      if (j < 3) then
        ! Special case for j < 3
        j = 1
        do i = 2, n
          if (p(i-1) > p(i)) then
            j = i + 1
          end if
        end do

        do i = 1, j-1
          k = i
          temp = p(i)
          do while (k < j-1)
            k = k + 1
            p(k-1) = p(k)
          end do
          p(j-1) = temp
        end do
      else
        j = j - 1
        first_j = p(j)
        do i = j, 2, -1
          p(i) = p(i-1)
        end do
        p(1) = first_j
      end if
    end do
  end function fannkuch_redux

end program fannkuch