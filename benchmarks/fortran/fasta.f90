! FASTA benchmark - generate and write random DNA sequences

program fasta
  use iso_fortran_env, only: dp => real64, stdout => output_unit
  implicit none

  ! Constants for the random number generator
  integer, parameter :: IM = 139968
  integer, parameter :: IA = 3877
  integer, parameter :: IC = 29573
  integer :: seed = 42

  ! Define DNA sequences
  character(len=287), parameter :: ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

  ! IUB probabilities and characters
  real(dp), dimension(15) :: IUB_PROB = &
       [0.27_dp, 0.12_dp, 0.12_dp, 0.27_dp, 0.02_dp, &
        0.02_dp, 0.02_dp, 0.02_dp, 0.02_dp, 0.02_dp, &
        0.02_dp, 0.02_dp, 0.02_dp, 0.02_dp, 0.02_dp]

  character(len=1), dimension(15) :: IUB_CHAR = &
       ['a', 'c', 'g', 't', 'B', &
        'D', 'H', 'K', 'M', 'N', &
        'R', 'S', 'V', 'W', 'Y']

  ! Homo sapiens probabilities and characters
  real(dp), dimension(4) :: HOMO_SAPIENS_PROB = &
       [0.3029549426680_dp, 0.1979883004921_dp, &
        0.1975473066391_dp, 0.3015094502008_dp]

  character(len=1), dimension(4) :: HOMO_SAPIENS_CHAR = &
       ['a', 'c', 'g', 't']

  ! Parameters
  integer, parameter :: n = 1000000
  integer :: i, start_time, end_time, count_rate
  character(len=n) :: result

  ! Start timing
  call system_clock(start_time, count_rate)

  ! Write FASTA header and sequence for Homo sapiens Alu
  write(stdout, '(a)') ">ONE Homo sapiens alu"
  call repeat_fasta(n, ALU, result)
  write(stdout, '(a)') trim(result)

  ! Write FASTA header and random sequence for IUB ambiguity codes
  write(stdout, '(a)') ">TWO IUB ambiguity codes"
  call gen_random_fasta(n, IUB_PROB, IUB_CHAR, size(IUB_PROB), result)
  write(stdout, '(a)') trim(result)

  ! Write FASTA header and random sequence for Homo sapiens frequency
  write(stdout, '(a)') ">THREE Homo sapiens frequency"
  call gen_random_fasta(n, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR, size(HOMO_SAPIENS_PROB), result)
  write(stdout, '(a)') trim(result)

  ! End timing and report
  call system_clock(end_time)
  write(stdout, '(a, i0, a)') "Time taken: ", (end_time - start_time) * 1000 / count_rate, " ms"

contains

  ! Generate a random number
  function gen_random() result(r)
    real(dp) :: r
    
    seed = mod(seed * IA + IC, IM)
    r = real(seed, dp) / real(IM, dp)
  end function gen_random

  ! Generate a random FASTA sequence
  subroutine gen_random_fasta(n, probs, chars, length, result)
    integer, intent(in) :: n, length
    real(dp), dimension(length), intent(in) :: probs
    character(len=1), dimension(length), intent(in) :: chars
    character(len=*), intent(out) :: result

    integer :: i, j
    real(dp) :: r, p
    character(len=1) :: c

    do i = 1, n
      r = gen_random()
      p = 0.0_dp
      
      do j = 1, length
        p = p + probs(j)
        if (r < p) then
          c = chars(j)
          exit
        end if
      end do

      result(i:i) = c
    end do
  end subroutine gen_random_fasta

  ! Repeat a sequence until it reaches the required length
  subroutine repeat_fasta(n, seq, result)
    integer, intent(in) :: n
    character(len=*), intent(in) :: seq
    character(len=*), intent(out) :: result

    integer :: i, seq_len

    seq_len = len_trim(seq)
    
    do i = 1, n
      result(i:i) = seq(mod(i-1, seq_len) + 1:mod(i-1, seq_len) + 1)
    end do
  end subroutine repeat_fasta

end program fasta