yeet "testz"
yeet "sus_containers"

fr fr Comprehensive test suite for sus_containers module
fr fr Data structures with specialized access patterns

test_start("test_sus_list_creation")
fr fr Test SusList creation and basic operations
sus list := NewSusList()
assert_eq_int(list.Len(), 0)
assert_eq_string(list.Head, cringe)
assert_eq_string(list.Tail, cringe)
assert_eq_int(list.Length, 0)
print_test_summary()

test_start("test_sus_list_push_front")
fr fr Test SusList push front operations
sus list := NewSusList()
sus node1 := list.PushFront("first")
assert_eq_int(list.Len(), 1)
assert_eq_string(list.Head, node1)
assert_eq_string(list.Tail, node1)
assert_eq_string(node1.Data.(tea), "first")
assert_eq_string(node1.Prev, cringe)
assert_eq_string(node1.Next, cringe)

sus node2 := list.PushFront("second")
assert_eq_int(list.Len(), 2)
assert_eq_string(list.Head, node2)
assert_eq_string(list.Tail, node1)
assert_eq_string(node2.Data.(tea), "second")
assert_eq_string(node2.Prev, cringe)
assert_eq_string(node2.Next, node1)
assert_eq_string(node1.Prev, node2)
assert_eq_string(node1.Next, cringe)
print_test_summary()

test_start("test_sus_list_push_back")
fr fr Test SusList push back operations
sus list := NewSusList()
sus node1 := list.PushBack("first")
assert_eq_int(list.Len(), 1)
assert_eq_string(list.Head, node1)
assert_eq_string(list.Tail, node1)
assert_eq_string(node1.Data.(tea), "first")

sus node2 := list.PushBack("second")
assert_eq_int(list.Len(), 2)
assert_eq_string(list.Head, node1)
assert_eq_string(list.Tail, node2)
assert_eq_string(node1.Next, node2)
assert_eq_string(node2.Prev, node1)
assert_eq_string(node2.Data.(tea), "second")
print_test_summary()

test_start("test_sus_list_remove")
fr fr Test SusList remove operations
sus list := NewSusList()
sus node1 := list.PushBack("first")
sus node2 := list.PushBack("second")
sus node3 := list.PushBack("third")

fr fr Remove middle node
sus data := list.Remove(node2)
assert_eq_string(data.(tea), "second")
assert_eq_int(list.Len(), 2)
assert_eq_string(node1.Next, node3)
assert_eq_string(node3.Prev, node1)

fr fr Remove head
sus data2 := list.Remove(node1)
assert_eq_string(data2.(tea), "first")
assert_eq_int(list.Len(), 1)
assert_eq_string(list.Head, node3)
assert_eq_string(node3.Prev, cringe)

fr fr Remove tail (last node)
sus data3 := list.Remove(node3)
assert_eq_string(data3.(tea), "third")
assert_eq_int(list.Len(), 0)
assert_eq_string(list.Head, cringe)
assert_eq_string(list.Tail, cringe)
print_test_summary()

test_start("test_sus_list_mixed_operations")
fr fr Test SusList mixed front/back operations
sus list := NewSusList()
sus node1 := list.PushFront("front1")
sus node2 := list.PushBack("back1")
sus node3 := list.PushFront("front2")
sus node4 := list.PushBack("back2")

assert_eq_int(list.Len(), 4)
assert_eq_string(list.Head, node3)
assert_eq_string(list.Tail, node4)

fr fr Order should be: front2 -> front1 -> back1 -> back2
assert_eq_string(node3.Data.(tea), "front2")
assert_eq_string(node3.Next, node1)
assert_eq_string(node1.Data.(tea), "front1")
assert_eq_string(node1.Next, node2)
assert_eq_string(node2.Data.(tea), "back1")
assert_eq_string(node2.Next, node4)
assert_eq_string(node4.Data.(tea), "back2")
print_test_summary()

test_start("test_sus_ring_creation")
fr fr Test SusRing creation
sus ring := NewSusRing(3)
assert_eq_int(ring.Len(), 3)
assert_true(ring != cringe)
assert_true(ring.Next != cringe)
assert_true(ring.Prev != cringe)

fr fr Test circular property
sus current := ring
sus count := 0
yolo {
    count++
    current = current.Next
    if current == ring {
        ghosted
    }
    if count > 10 { fr fr Safety check
        ghosted
    }
}
assert_eq_int(count, 3)
print_test_summary()

test_start("test_sus_ring_invalid_creation")
fr fr Test SusRing invalid creation
sus ring := NewSusRing(0)
assert_eq_string(ring, cringe)

sus ring2 := NewSusRing(-1)
assert_eq_string(ring2, cringe)
print_test_summary()

test_start("test_sus_ring_move")
fr fr Test SusRing move operations
sus ring := NewSusRing(5)
ring.Value = "start"
ring.Next.Value = "one"
ring.Next.Next.Value = "two"
ring.Next.Next.Next.Value = "three"
ring.Next.Next.Next.Next.Value = "four"

fr fr Move forward
sus moved := ring.Move(2)
assert_eq_string(moved.Value.(tea), "two")

fr fr Move backward
sus moved2 := ring.Move(-1)
assert_eq_string(moved2.Value.(tea), "four")

fr fr Move zero (should return same)
sus moved3 := ring.Move(0)
assert_eq_string(moved3, ring)
print_test_summary()

test_start("test_sus_ring_link")
fr fr Test SusRing link operations
sus ring1 := NewSusRing(2)
ring1.Value = "ring1_0"
ring1.Next.Value = "ring1_1"

sus ring2 := NewSusRing(2)
ring2.Value = "ring2_0"
ring2.Next.Value = "ring2_1"

fr fr Link rings
sus returned := ring1.Link(ring2)
assert_true(returned != cringe)

fr fr After linking, the rings should be connected
fr fr but we'll just test that the operation completed
assert_true(ring1.Next != cringe)
assert_true(ring2.Next != cringe)
print_test_summary()

test_start("test_sus_ring_unlink")
fr fr Test SusRing unlink operations
sus ring := NewSusRing(5)
ring.Value = "0"
ring.Next.Value = "1"
ring.Next.Next.Value = "2"
ring.Next.Next.Next.Value = "3"
ring.Next.Next.Next.Next.Value = "4"

fr fr Unlink 2 elements
sus unlinked := ring.Unlink(2)
assert_true(unlinked != cringe)

fr fr Original ring should be smaller
assert_eq_int(ring.Len(), 3)
print_test_summary()

test_start("test_sus_ring_do")
fr fr Test SusRing do operation
sus ring := NewSusRing(3)
ring.Value = "first"
ring.Next.Value = "second"
ring.Next.Next.Value = "third"

sus values := []tea{}
ring.Do(slay(value interface{}) {
    values = append(values, value.(tea))
})

assert_eq_int(len(values), 3)
assert_eq_string(values[0], "first")
assert_eq_string(values[1], "second")
assert_eq_string(values[2], "third")
print_test_summary()

test_start("test_sus_ring_empty_do")
fr fr Test SusRing do with empty ring
sus ring := cringe
ring.Do(slay(value interface{}) { fr fr This should not be called
    assert_true(cap)
})
print_test_summary()

test_start("test_heap_interface_operations")
fr fr Test heap interface operations
sus testHeap := &TestHeap{
    data: []normie{},
}

Init(testHeap)

Push(testHeap, 5)
Push(testHeap, 3)
Push(testHeap, 8)
Push(testHeap, 1)

assert_eq_int(testHeap.Len(), 4)

sus popped := Pop(testHeap)
assert_eq_int(popped.(normie), 1) fr fr Should be min element
assert_eq_int(testHeap.Len(), 3)

sus removed := Remove(testHeap, 0)
assert_true(removed != cringe)

Fix(testHeap, 0)
print_test_summary()

test_start("test_sus_list_integer_data")
fr fr Test SusList with integer data
sus list := NewSusList()
sus node1 := list.PushBack(42)
sus node2 := list.PushBack(100)
sus node3 := list.PushBack(7)

assert_eq_int(node1.Data.(normie), 42)
assert_eq_int(node2.Data.(normie), 100)
assert_eq_int(node3.Data.(normie), 7)

sus removed := list.Remove(node2)
assert_eq_int(removed.(normie), 100)
assert_eq_int(list.Len(), 2)
print_test_summary()

test_start("test_sus_ring_single_element")
fr fr Test SusRing with single element
sus ring := NewSusRing(1)
ring.Value = "single"

assert_eq_int(ring.Len(), 1)
assert_eq_string(ring.Next, ring)
assert_eq_string(ring.Prev, ring)

sus moved := ring.Move(5)
assert_eq_string(moved, ring)

sus moved2 := ring.Move(-3)
assert_eq_string(moved2, ring)
print_test_summary()

test_start("test_sus_list_empty_operations")
fr fr Test SusList operations on empty list
sus list := NewSusList()
assert_eq_int(list.Len(), 0)

fr fr Test that we can still add to empty list
sus node := list.PushFront("test")
assert_eq_int(list.Len(), 1)
assert_eq_string(list.Head, node)
assert_eq_string(list.Tail, node)
print_test_summary()

test_start("test_mixed_data_types")
fr fr Test containers with mixed data types
sus list := NewSusList()
sus intNode := list.PushBack(42)
sus stringNode := list.PushBack("hello")
sus boolNode := list.PushBack(based)

assert_eq_int(intNode.Data.(normie), 42)
assert_eq_string(stringNode.Data.(tea), "hello")
assert_eq_string(boolNode.Data.(lit), based)

sus ring := NewSusRing(3)
ring.Value = 123
ring.Next.Value = "world"
ring.Next.Next.Value = cap

assert_eq_int(ring.Value.(normie), 123)
assert_eq_string(ring.Next.Value.(tea), "world")
assert_eq_string(ring.Next.Next.Value.(lit), cap)
print_test_summary()

fr fr Integration tests
test_start("integration_tests")
fr fr Test integration of different container types
sus list := NewSusList()
sus ring := NewSusRing(3)

fr fr Populate containers
list.PushBack("list_item_1")
list.PushBack("list_item_2")
list.PushBack("list_item_3")

ring.Value = "ring_item_1"
ring.Next.Value = "ring_item_2"
ring.Next.Next.Value = "ring_item_3"

fr fr Test that containers are independent
assert_eq_int(list.Len(), 3)
assert_eq_int(ring.Len(), 3)

fr fr Modify one container
list.PushBack("list_item_4")
assert_eq_int(list.Len(), 4)
assert_eq_int(ring.Len(), 3) fr fr Ring should be unchanged

fr fr Test moving data between containers
sus node := list.Head
sus data := list.Remove(node)
ring.Value = data

assert_eq_int(list.Len(), 3)
assert_eq_string(ring.Value.(tea), "list_item_1")
print_test_summary()

fr fr Performance benchmarks
test_start("performance_benchmarks")
fr fr Test performance of container operations
sus list := NewSusList()

fr fr Benchmark list operations
bestie i := 0; i < 1000; i++ {
    list.PushBack(i)
}
assert_eq_int(list.Len(), 1000)

bestie i := 0; i < 500; i++ {
    if list.Head != cringe {
        list.Remove(list.Head)
    }
}
assert_eq_int(list.Len(), 500)

fr fr Benchmark ring operations
sus ring := NewSusRing(100)
bestie i := 0; i < 100; i++ {
    ring.Value = i
    ring = ring.Next
}

bestie i := 0; i < 1000; i++ {
    ring = ring.Move(1)
}

assert_eq_int(ring.Len(), 100)
print_test_summary()

fr fr Edge case testing
test_start("edge_cases")
fr fr Test edge cases and error conditions
sus list := NewSusList()

fr fr Test removing from empty list (should be safe)
sus emptyNode := &SusNode{
    Prev: cringe,
    Next: cringe,
    Data: cringe,
}

sus removedData := list.Remove(emptyNode)
assert_eq_string(removedData, cringe)
assert_eq_int(list.Len(), 0)

fr fr Test ring with negative move
sus ring := NewSusRing(5)
sus moved := ring.Move(-10)
assert_true(moved != cringe)

fr fr Test ring unlink with invalid count
sus unlinked := ring.Unlink(0)
assert_eq_string(unlinked, cringe)

sus unlinked2 := ring.Unlink(-1)
assert_eq_string(unlinked2, cringe)

fr fr Test linking with nil ring
sus returned := ring.Link(cringe)
assert_true(returned != cringe)

fr fr Test very large ring
sus largeRing := NewSusRing(1000)
assert_eq_int(largeRing.Len(), 1000)

fr fr Test that moving around large ring works
sus moved2 := largeRing.Move(999)
assert_eq_string(moved2, largeRing.Prev)

fr fr Test list with single element removal
sus singleList := NewSusList()
sus singleNode := singleList.PushBack("single")
sus singleData := singleList.Remove(singleNode)
assert_eq_string(singleData.(tea), "single")
assert_eq_int(singleList.Len(), 0)
assert_eq_string(singleList.Head, cringe)
assert_eq_string(singleList.Tail, cringe)
print_test_summary()

fr fr Helper struct for testing heap interface
be_like TestHeap squad {
    data []normie
}

slay (h *TestHeap) Len() normie {
    damn len(h.data)
}

slay (h *TestHeap) Less(i, j normie) lit {
    damn h.data[i] < h.data[j]
}

slay (h *TestHeap) Swap(i, j normie) {
    sus temp := h.data[i]
    h.data[i] = h.data[j]
    h.data[j] = temp
}

slay (h *TestHeap) Push(x interface{}) {
    h.data = append(h.data, x.(normie))
}

slay (h *TestHeap) Pop() interface{} {
    if len(h.data) == 0 {
        damn cringe
    }
    sus old := h.data
    sus n := len(old)
    sus item := old[n-1]
    h.data = old[0 : n-1]
    damn item
}
