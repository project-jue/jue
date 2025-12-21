The way Lisp (and particularly its most famous dialects like Common Lisp and Scheme) handles recursion is central to its identity and efficiency. There isn't a single "Lisp VM," but the principles are consistent across implementations (bytecode VMs like Emacs Lisp's, or native compilers like SBCL's).

Here’s a breakdown of how recursion is typically handled, focusing on the critical concept of **Tail Call Optimization (TCO)**.

### 1. The Naïve (Dangerous) Way: Growing the Call Stack
Without optimization, every recursive call creates a new **stack frame**, containing the function's arguments, return address, and local state.

```lisp
(defun factorial (n)
  (if (<= n 1)
      1
      (* n (factorial (- n 1))))) ; Recursive call is NOT in tail position
```
**Execution of `(factorial 3)` would look like this on the stack:**
```
(factorial 3)
  (* 3 (factorial 2))
            (factorial 2)
              (* 2 (factorial 1))
                        (factorial 1) -> returns 1
              (* 2 1) -> returns 2
  (* 3 2) -> returns 6
```
Each call must wait for the inner call to return so it can multiply the result by `n`. This consumes O(n) stack space and will fail with a **stack overflow** for large `n`.

### 2. The Lisp Way: Tail Recursion and Optimization (TCO)
The key is writing functions in **tail-recursive** form, where the recursive call is the *last* operation performed (the "tail" position).

```lisp
(defun factorial-tail (n &optional (acc 1))
  (if (<= n 1)
      acc
      (factorial-tail (- n 1) (* n acc)))) ; Recursive call IS in tail position.
```
Here, once `(factorial-tail (- n 1) (* n acc))` is ready to be called, there is **nothing left to do** in the current function frame. Its job is done.

#### **Tail Call Optimization (TCO) / Tail Call Elimination**
A **proper Lisp/Scheme VM or compiler** performs this crucial optimization:
1.  It identifies that the recursive call is in the **tail position**.
2.  Before making the new call, it **discards or reuses the current stack frame**.
3.  It jumps (`goto`) to the beginning of the function with the new arguments.

**Optimized execution of `(factorial-tail 3)` would look like this:**
```
Call #1: (factorial-tail 3 1)
  Frame is recycled. Jump to start with new args.
Call #2: (factorial-tail 2 3)  ; (- 3 1)=2, (* 3 1)=3
  Frame is recycled. Jump to start with new args.
Call #3: (factorial-tail 1 6)  ; (- 2 1)=1, (* 2 3)=6
  (<= n 1) is true, return 6.
```
**Crucially, only one stack frame is ever used.** This transforms the recursion into an **iteration at the machine level**. It runs in **O(1) space**.

### How the VM/Compiler Implements This
The transformation is essentially a **goto with argument update** in a loop. The compiler might translate the tail-recursive Lisp code into machine code like this:

```c
int factorial_tail(int n, int acc) {
  loop:
    if (n <= 1) return acc;
    acc = n * acc;
    n = n - 1;
    goto loop; // This is the tail-recursive call
}
```

### 3. Language Standards and Variations
*   **Scheme:** The language **requires** implementations to perform **proper tail recursion** (PTR) for all tail calls, not just recursion. It's a core part of the language specification.
*   **Common Lisp:** The standard *suggests* (`OPTIMIZE (SPEED 3) (DEBUG 0)`) that compilers should implement TCO, but it's not *required*. In practice, all major optimizing compilers (SBCL, CCL, etc.) do perform TCO at high optimization levels. The lack of a strict guarantee is due to the desire to support deep stack traces for debugging.
*   **Clojure (on the JVM):** Since the JVM doesn't natively support TCO, Clojure provides the `recur` special form for explicit tail recursion and uses the `loop` construct for iteration.

### 4. Beyond Recursion: General Tail Calls
In Scheme and optimized Lisps, this isn't just for recursion. Any call in the tail position is optimized:
```lisp
(defun foo (x)
  (bar (baz x))) ; The call to `bar` is in the tail position.

(defun bar (x)
  (cond ((a x) (foo (c x))) ; Tail call to `foo`
        ((b x) (bar (d x))) ; Tail call to itself (tail recursion)
        (t     (qux x))))   ; Tail call to `qux`
```
This enables powerful, stack-safe **continuation-passing style (CPS)** programming.

### Summary Table

| Feature              | Naïve Recursion            | Tail Recursion (without TCO) | Tail Recursion (with TCO)     |
| :------------------- | :------------------------- | :--------------------------- | :---------------------------- |
| **Stack Frames**     | O(n)                       | O(n)                         | **O(1)**                      |
| **Space Complexity** | Linear                     | Linear                       | **Constant**                  |
| **Mechanism**        | Call-return, nested frames | Call-return, nested frames   | **Frame reuse / Jump (Goto)** |
| **Risk**             | Stack Overflow             | Stack Overflow               | **None (for depth)**          |
| **Semantics**        | Recursion                  | Recursion                    | **Iteration**                 |

**In essence:** The Lisp VM/compiler doesn't handle recursion in a special way. Instead, it treats the **tail call** (which can be recursive or not) as a fundamental, optimizable operation. By reusing the stack frame, it turns recursive tail calls into efficient, safe loops. This makes recursion a practical and idiomatic replacement for iteration in Lisp.