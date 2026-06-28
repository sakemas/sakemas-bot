---
name: adr-review
description: Evaluate major architectural decisions against encapsulation, separation of concerns, invariants, consistency, and fitness for purpose.
---

# ADR Review

## Purpose

Evaluate whether a proposed architectural change justifies modifying the current design. Use this skill before accepting or recording a major decision.

Trigger this skill for:

- New dependencies or runtime targets
- Database schema or persistence changes
- Public API or command surface changes
- New subsystems or execution domains
- Major refactors
- Deployment or infrastructure changes

## Principle

The burden of proof is on the change. Existing design should be assumed sufficient until demonstrated otherwise. Start from a problem, not a preferred solution.

## Review Procedure

### Step 1: Describe the Problem

State:

- Current behavior
- Current limitation or observed issue
- Measured evidence, if available

Separate facts from assumptions.

### Step 2: Evaluate Existing Design

Determine whether the problem can be solved by:

- Existing modules or abstractions
- Existing data, invariant, API, or execution boundaries
- Existing extension points

Prefer extending an existing concept over creating a new one. Document why existing solutions are insufficient before proposing a new one.

### Step 3: Generate Alternatives

Identify at least:

- Minimum-change approach
- Moderate-change approach
- Large-scale redesign

For each, describe benefits, drawbacks, boundary impact, and complexity impact.

### Step 4: Evaluate Costs

Consider:

- Migration complexity
- Compatibility impact
- Documentation and design-document updates
- Testing impact
- Operational impact

### Step 5: Apply Required Axes

Evaluate the chosen alternative against:

- **Encapsulation:** internal details remain behind the owning boundary
- **Separation of concerns:** concerns do not bleed across module boundaries
- **Invariants:** the owner of an invariant enforces it at write boundaries
- **Consistency:** duplicated facts stay synchronized through one source of truth
- **Fitness for purpose:** the abstraction matches the concrete problem size

## Output Format

### Context

Where the decision sits in the repository.

### Problem

The demonstrated problem.

### Existing Design Assessment

Why existing concepts are insufficient, if applicable.

### Alternatives

At least three approaches with trade-offs.

### Recommendation

APPROVE / APPROVE WITH CHANGES / REJECT

### Rationale

Why the recommendation follows from the evidence.

### Required Axes Impact

How the change affects encapsulation, separation of concerns, invariants, consistency, and fitness for purpose.

### Required Design Updates

Which documents must be updated if the change proceeds.
