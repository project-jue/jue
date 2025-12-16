# **Non-Axiomatic Reasoning System (NARS): A Replication Blueprint**

NARS is a unified **cognitive architecture** built on the **Assumption of Insufficient Knowledge and Resources (AIKR)**. It's not a collection of algorithms, but a single reasoning system where learning, perception, and action emerge from one core process: **adaptation by belief revision**.

## **1. Foundational Philosophy**

**Core Tenet:** Intelligence is the ability for a system to **adapt to its environment** with **finite knowledge** and **bounded resources**.

*   **Non-Axiomatic:** It has no fixed, unquestionable "axioms." All beliefs are **revisable** based on new evidence.
*   **Experience-Grounded:** Meaning is defined **operationally** by how a term is used in the system's experience, not by external semantics.
*   **Goal-Oriented:** The system's "desires" are internal goals it must satisfy by deriving tasks.

## **2. The Core Data Structure: The *Term***

Everything is a **Term**. A Term is a concept identified by a unique name. There are three atomic types, from which all others are built:

1.  **Independent Term:** A primitive symbol (e.g., `cat`, `swim`).
2.  **Compound Term:** Formed by connectors. **This is NARS's "language of thought."**
    *   **Intersection Extensional (`∧`):** `cat∧black` ("black cat").
    *   **Intersection Intensional (`∩`):** `swim∩fly` ("things that can swim and can fly").
    *   **Difference (`-`):** `animal-cat` ("animals that are not cats").
    *   **Product (`×`):** `Paris×France` (an ordered pair, for relations).
    *   **Image (`/` and `\`):** `(×, agent, Paris)` ↔ `agent×(×, _, Paris)` (for partial relations).

3.  **Statement (Special Compound):** A relation between two Terms, forming a belief. `S → P`.
    *   **Inheritance (`→`):** `cat → animal` ("Cats are a type of animal"). The fundamental relation.
    *   **Similarity (`↔`):** `cat ↔ feline` (Symmetric inheritance).
    *   **Instance (`◦→`):** `Felix ◦→ cat` ("Felix is an instance of a cat").
    *   **Property (`→◦`):** `cat →◦ furry` ("Cats have the property of being furry").
    *   **Implication (`⇒`):** `(rain ∧ windy) ⇒ stormy` (Event/ Temporal relations).

## **3. Truth & Evidence: The *Truth-Value***

A truth-value is not Boolean. It's a continuous measure of **evidential support**.

*   **Frequency (`f` ∈ [0, 1]):** `w+ / w` - The proportion of *positive* evidence (`w+`) to *total* evidence (`w`). It answers "How often has this been true?"
*   **Confidence (`c` ∈ [0, 1]):** `w / (w + k)` - How much the system trusts the frequency. `k` is a constant (usually 1). More evidence (`w`) → higher confidence.

A Truth-Value is the pair `(f, c)`. A belief is a **Statement tagged with a Truth-Value**.

**Evidence is defined operationally:**
*   For `S → P`:
    *   **Positive Evidence:** An instance that is both `S` and `P`.
    *   **Negative Evidence:** An instance that is `S` but **not** `P`.
*   There is no "absolute truth," only truth relative to the system's limited experience.

## **4. Memory: *Bag* and *Concepts***

*   **The *Bag*:** The system's working memory. It's a **probabilistic priority queue** containing:
    *   **Tasks:** (Statement, Truth-Value, Type). A Type is a *Judgment* (to be believed), *Goal* (to be achieved), or *Question* (to be answered).
    *   **Active Concepts:** Concepts currently being processed.
*   **Concept:** The system's long-term memory unit for a single Term. Each Concept `C` contains:
    *   `Beliefs`: A set of judgments about `C`, sorted by confidence and relevance.
    *   `Goals`: A set of desired states involving `C`.
    *   `Questions`: Pending queries about `C`.
    *   `Links`: Pointers to related Concepts (for efficient retrieval).

## **5. The Inference Engine: *Formal Rules***

Inference is **term logic** deduction, revision, and induction over uncertain truths. Rules are **syllogistic**: they take two premises with a common term and derive a conclusion.

Key Rules (with Truth-Value Functions):

**1. Deduction (Strong Inference):**
```
Given: M → P <f1, c1> AND S → M <f2, c2>
Conclude: S → P <F_ded(f1,c1, f2,c2), C_ded(c1,c2)>
```
*Function `F_ded`: ~ f1 * f2. `C_ded`: ~ c1 * c2.*

**2. Revision (Belief Merging):**
```
Given: S → P <f1, c1> AND S → P <f2, c2> (Same statement)
Conclude: S → P <F_rev(f1,w1, f2,w2), C_rev(w1,w2)>
```
*Where `w = k / (1-c)` is the evidence weight. The new frequency is the evidence-weighted average.*

**3. Induction (Abduction-like):**
```
Given: M → P <f1, c1> AND S → M <f2, c2>
Conclude: S → P <F_ind(f1,c1, f2,c2), C_ind(c1,c2)>
```
*Function `F_ind`: ~ f2. Lower confidence. This allows learning generalizations from single examples.*

**4. Abduction (like Induction but other direction).**
**5. Comparison (derive Similarity).**
**6. Analogy (use Similarity as a "soft" middle term).**

**Every rule has a precise truth-function that calculates the new `(f, c)` from the old.** These functions are derived from **probability theory** and the evidential interpretation.

## **6. The Control Cycle: *Working Process***

The system runs in a perpetual loop. One **processing cycle** works as follows:

1.  **SELECT:** Take an item (a Task or a Concept) from the *Bag*. Selection is **probabilistic**, weighted by priority (`Priority = urgency * durability`).
2.  **EXECUTE:**
    *   If it's a **Task** for Concept `C`, access Concept `C` and try to **process the Task** (e.g., derive new beliefs using its beliefs).
    *   If it's a **Concept** `C`, process its internal beliefs/goals against each other.
3.  **INFER:** Apply all relevant inference rules between the selected item and the contents of the active Concept. Each derived conclusion becomes a **new Task**.
4.  **UPDATE:**
    *   Add new Tasks to the *Bag*.
    *   Revise the priority of all involved items (used items decay slightly; novel/important items get boosted).
    *   Manage memory: if the *Bag* is full, remove the lowest-priority items.

## **7. Input, Output, and Meaning**

*   **Input:** A stream of `Narsese` sentences.
    *   `A → B. <f, c>` : A judgment to be absorbed (e.g., "Robins are birds, 0.9 confident").
    *   `A → B?` : A question.
    *   `A → B! <f,c>` : A goal (e.g., "Be near door, desirability 0.8").
*   **Output:** Derived judgments, answers to questions, or **executable operations**.
    *   `^op(arg)` is a special Term representing an action. If `^op(arg)!` becomes a high-priority goal, the system executes it in its environment.
*   **Meaning:** The meaning of a Term is its **procedural and associative relations** within the network—its links, its beliefs, and its role in derived tasks. It's **fluid** and evolves.

## **8. Detailed Replication Guide**

To build a working NARS from scratch, follow these modules:

**Module 1: Term & Statement Representation**
*   Implement the Term hierarchy (Atomic, Compound).
*   Implement Statement connectors (`→`, `∧`, `⇒`, etc.) as syntax trees.
*   Implement the Truth-Value struct `{f: float, c: float}` and the `w` (weight) calculator.

**Module 2: Memory Structures**
*   Implement the `Concept` struct with belief/goal/question bags. Use sorted vectors or priority heaps.
*   Implement the global `Bag`. Use a stochastic data structure (like a "priority wheel" or algorithm `R`).
*   Implement a hash map (`TermString` → `Concept`) for the whole memory.

**Module 3: Truth-Function Library**
*   Code the exact truth-functions for all inference rules. Start with:
    *   `F_ded`, `C_ded`
    *   `F_rev`, `C_rev` (This is critical)
    *   `F_ind`, `C_ind`
*   These functions are deterministic and purely mathematical.

**Module 4: Inference Engine**
*   For a given pair of premises (`S1`, `S2`), write a function to:
    1.  **Match:** Find the overlapping Term between `S1` and `S2`.
    2.  **Select Rule:** Determine which syllogistic rule applies.
    3.  **Calculate:** Call the corresponding truth-function.
    4.  **Construct Conclusion:** Build the new Statement with its new Truth-Value.

**Module 5: Control Cycle & Scheduler**
*   Implement the main loop:
    ```rust
    loop {
        let budget = 1.0; // Total resources per cycle
        while budget > 0.0 {
            let (item, priority) = bag.select(); // Stochastic selection
            let concept = memory.get_concept(item.term);
            let derived_tasks = process(concept, item);
            budget -= priority; // Deduct cost
            for task in derived_tasks {
                bag.insert(task, initial_priority);
            }
            update_priorities(item, concept); // Decay & boost
        }
        memory.cleanup(); // Forget low-priority concepts
    }
    ```

**Module 6: Parser & Interface**
*   Implement `Narsese` parser for input: `(statement, truth-value, punctuation)`.
    *   Punctuation: `.` (judgment), `?` (question), `!` (goal).
*   Hook `^op` terms to external function calls.

## **9. Critical Design Choices for Your Implementation**

1.  **Forgetfulness:** The `Bag` and priority decay are your **resource management**. Without it, you have a logic engine, not NARS.
2.  **Evidence Counting:** Strictly track `w` (positive and total) for every belief to enable correct revision.
3.  **Real-time Operation:** The control cycle must process a **small, fixed amount** per iteration to guarantee responsiveness. Never run inference to completion.
4.  **Start Small:** Begin with Inheritance, Deduction, and Revision only. Add compound terms and other rules later.

This architecture yields a system that:
*   **Learns continuously** from a single example (via Induction/Revision).
*   **Answers questions** under uncertainty ("Probably, but I'm not sure").
*   **Exhibits "cognitive" behaviors:** forgetting, focus, analogy, and goal-directed reasoning—all from one core mechanism.

To integrate with **Project Jue**, NARS would become a **primary reasoning module within Dan-World**. Its `(f, c)` beliefs are the perfect embodiment of *Belief Facets*, and its term logic provides the "language" for Dan's internal model. The Physics World would execute its `^op` actions, and Jue World could compile higher-level plans into Narsese task streams.