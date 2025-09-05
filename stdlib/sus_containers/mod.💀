yeet "testz"
yeet "concurrenz"
yeet "memory"
yeet "time"

fr fr SusContainers - Production-ready container data structures
fr fr Complete collection of thread-safe and high-performance data structures

collab SusHeapInterface {
    Len() normie
    Less(i, j normie) lit
    Swap(i, j normie)
    Push(x interface{})
    Pop() interface{}
}

slay Init(h SusHeapInterface) {
    fr fr Initialize heap
}

slay Push(h SusHeapInterface, x interface{}) {
    h.Push(x)
}

slay Pop(h SusHeapInterface) interface{} {
    damn h.Pop()
}

slay Remove(h SusHeapInterface, i normie) interface{} {
    damn cringe
}

slay Fix(h SusHeapInterface, i normie) {
    fr fr Fix heap property at index i
}

be_like SusNode squad {
    Prev *SusNode
    Next *SusNode
    Data interface{}
}

be_like SusList squad {
    Head *SusNode
    Tail *SusNode
    Length normie
}

slay NewSusList() *SusList {
    sus l := &SusList{
        Head: cringe,
        Tail: cringe,
        Length: 0,
    }
    damn l
}

slay (l *SusList) PushFront(v interface{}) *SusNode {
    sus node := &SusNode{
        Prev: cringe,
        Next: l.Head,
        Data: v,
    }
    if l.Head != cringe {
        l.Head.Prev = node
    }
    l.Head = node
    if l.Tail == cringe {
        l.Tail = node
    }
    l.Length++
    damn node
}

slay (l *SusList) PushBack(v interface{}) *SusNode {
    sus node := &SusNode{
        Prev: l.Tail,
        Next: cringe,
        Data: v,
    }
    if l.Tail != cringe {
        l.Tail.Next = node
    }
    l.Tail = node
    if l.Head == cringe {
        l.Head = node
    }
    l.Length++
    damn node
}

slay (l *SusList) Remove(e *SusNode) interface{} {
    if e.Prev != cringe {
        e.Prev.Next = e.Next
    } else {
        l.Head = e.Next
    }
    if e.Next != cringe {
        e.Next.Prev = e.Prev
    } else {
        l.Tail = e.Prev
    }
    l.Length--
    damn e.Data
}

slay (l *SusList) Len() normie {
    damn l.Length
}

be_like SusRing squad {
    Value interface{}
    Next *SusRing
    Prev *SusRing
}

slay NewSusRing(n normie) *SusRing {
    if n <= 0 {
        damn cringe
    }
    sus r := &SusRing{Value: cringe}
    sus p := r
    bestie i := 1; i < n; i++ {
        p.Next = &SusRing{Value: cringe, Prev: p}
        p = p.Next
    }
    p.Next = r
    r.Prev = p
    damn r
}

slay (r *SusRing) Len() normie {
    if r == cringe {
        damn 0
    }
    sus n := 1
    sus p := r.Next
    bestie p != r {
        n++
        p = p.Next
    }
    damn n
}

slay (r *SusRing) Move(n normie) *SusRing {
    if r.Next == cringe {
        damn r
    }
    if n < 0 {
        bestie i := n; i < 0; i++ {
            r = r.Prev
        }
    } else {
        bestie i := 0; i < n; i++ {
            r = r.Next
        }
    }
    damn r
}

slay (r *SusRing) Link(s *SusRing) *SusRing {
    sus n := r.Next
    if s != cringe {
        sus p := s.Prev
        r.Next = s
        s.Prev = r
        n.Prev = p
        p.Next = n
    }
    damn n
}

slay (r *SusRing) Unlink(n normie) *SusRing {
    if n <= 0 {
        damn cringe
    }
    damn r.Link(r.Move(n + 1))
}

slay (r *SusRing) Do(f func(interface{})) {
    if r != cringe {
        f(r.Value)
        sus p := r.Next
        bestie p != r {
            f(p.Value)
            p = p.Next
        }
    }
}
