! Binary trees benchmark adapted from The Computer Language Benchmarks Game

program binary_trees
  use iso_fortran_env, only: stdout => output_unit
  implicit none

  type :: tree_node
    integer :: item
    type(tree_node), pointer :: left => null()
    type(tree_node), pointer :: right => null()
  end type tree_node

  integer, parameter :: min_depth = 4
  integer, parameter :: max_depth = 12
  integer :: stretch_depth, depth, iterations, i, result, check
  integer :: start_time, end_time, count_rate
  type(tree_node), pointer :: stretch_tree, long_lived_tree, temp_tree_a, temp_tree_b

  stretch_depth = max_depth + 1
  call system_clock(start_time, count_rate)

  ! Allocate and check a big tree
  stretch_tree => new_tree(0, stretch_depth)
  write(stdout, '(A,I0,A,I0)') "stretch tree of depth ", stretch_depth, " check: ", check_tree(stretch_tree)
  call deallocate_tree(stretch_tree)

  ! Allocate a long-lived binary tree
  long_lived_tree => new_tree(0, max_depth)

  ! Check trees of increasing depth
  do depth = min_depth, max_depth, 2
    iterations = 2 ** (max_depth - depth + min_depth)
    result = 0

    do i = 1, iterations
      temp_tree_a => new_tree(i, depth)
      temp_tree_b => new_tree(-i, depth)
      result = result + check_tree(temp_tree_a) + check_tree(temp_tree_b)
      call deallocate_tree(temp_tree_a)
      call deallocate_tree(temp_tree_b)
    end do

    write(stdout, '(I0,A,I0,A,I0)') iterations * 2, " trees of depth ", depth, " check: ", result
  end do

  ! Check the long-lived tree last
  write(stdout, '(A,I0,A,I0)') "long lived tree of depth ", max_depth, " check: ", check_tree(long_lived_tree)
  call deallocate_tree(long_lived_tree)

  call system_clock(end_time)
  write(stdout, '(A,I0,A)') "Time taken: ", (end_time - start_time) * 1000 / count_rate, " ms"

contains

  recursive function new_tree(item, depth) result(node)
    integer, intent(in) :: item, depth
    type(tree_node), pointer :: node
    
    allocate(node)
    node%item = item
    
    if (depth > 0) then
      node%left => new_tree(2 * item - 1, depth - 1)
      node%right => new_tree(2 * item, depth - 1)
    end if
  end function new_tree

  recursive function check_tree(node) result(check)
    type(tree_node), pointer, intent(in) :: node
    integer :: check
    
    if (.not. associated(node)) then
      check = 0
      return
    end if
    
    if (.not. associated(node%left)) then
      check = node%item
    else
      check = node%item + check_tree(node%left) - check_tree(node%right)
    end if
  end function check_tree

  recursive subroutine deallocate_tree(node)
    type(tree_node), pointer, intent(inout) :: node
    
    if (.not. associated(node)) return
    
    if (associated(node%left)) call deallocate_tree(node%left)
    if (associated(node%right)) call deallocate_tree(node%right)
    
    deallocate(node)
    node => null()
  end subroutine deallocate_tree

end program binary_trees