# Functional Autonomy Contract

## Scope

This project implements functional autonomy: persistent user goals, bounded
reviews, evidence-driven completion, and controlled learning. It does not
claim consciousness, subjective experience, free will, or self-authorized
external action.

## Design sources and decisions

| Source | Engineering decision |
|---|---|
| [Principles of Metareasoning](https://doi.org/10.1016/0004-3702(91)90015-C) | additional computation requires a budget and an expected purpose |
| [MIDCA](https://github.com/colab2/MIDCA) | use a distinct meta layer that observes, assesses and guides; do not copy the archived Python 2.7 `mclumd/MIDCA` code |
| [Reflexion](https://github.com/noahshinn/reflexion) | retain a lesson only after an externally verified outcome and compare against a no-reflection baseline |
| [Generative Agents](https://arxiv.org/abs/2304.03442) | separate observation, reflection and planning; do not equate believable behavior with autonomy or consciousness |
| [Soar](https://soar.eecs.umich.edu/) | model goal state and impasses explicitly |
| [Butlin et al.](https://arxiv.org/abs/2308.08708) | keep consciousness claims outside engineering acceptance criteria |

MIDCA’s older `mclumd/MIDCA` repository is archived and documents Python 2.7.
It is a historical reference, not a runtime dependency.

## Goal protocol

```text
USER CREATES GOAL
       |
       v
GOAL MANAGER ASSESSES
       |
       +-- verified evidence ------> COMPLETE
       +-- insufficient budget ----> STOP
       +-- missing capability -----> STOP
       +-- otherwise --------------> REVIEW
```

A `REVIEW` directive is not execution. It cannot issue a tool call, grant a
capability, create another goal, or change a policy. External effects remain
subject to the typed action and permission transitions.

## Learning rule

```text
attempt -> observation -> independent verdict -> lesson candidate
                                                    |
                                evaluation on held-out tasks
                                                    |
                                      retain or discard
```

Unverified text is never promoted to durable knowledge or used to claim that a
strategy improved. The current `GoalManager` only manages goal state in memory;
SQLite persistence and a benchmarked lesson store remain future work.

## Acceptance criteria

1. A required capability cannot be bypassed by an active goal.
2. Only evidence bound to the goal’s exact objective can complete it.
3. A budget exhausted condition produces `STOP`, not an extra attempt.
4. No implementation test or document treats this protocol as evidence of
   subjective will or consciousness.
