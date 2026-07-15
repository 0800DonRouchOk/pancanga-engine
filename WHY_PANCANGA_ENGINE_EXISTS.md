# Why Pancanga Engine Exists

Many Vaiṣṇava communities depend on calendars whose results are trusted, used,
and lived by, but whose internal logic is not always easy to inspect.

Pancanga Engine exists to make that logic explicit.

Its purpose is not to replace a living tradition, nor to turn sacred practice
into a black-box program. Its purpose is to implement a clearly declared
methodology in a way that can be read, tested, audited, and traced.

For v1.0, the project follows a simple discipline:

```text
Sources
  -> Knowledge Base
  -> Specification
  -> Implementation
  -> Certification
```

The astronomical layer is built from explicit astronomical models and validated
against external astronomical references. The calendar layer consumes those
astronomical results without applying religious rules. The Vaiṣṇava layer
implements the rules documented in the Knowledge Base, whose primary normative
authority is the Hari-bhakti-vilāsa.

That separation matters.

It means that a disagreement can be studied without confusion:

```text
Is it astronomy?
Is it civil calendar configuration?
Is it normative interpretation?
Is it an external-oracle difference?
Is it an engine bug?
```

Pancanga Engine treats those questions separately. It does not change the
Knowledge Base to fit code. It does not change the engine merely to match an
external calendar. Every difference must first be classified.

The goal of v1.0 is therefore not to contain every possible feature of a full
pañcāṅga. The goal is narrower and stronger:

```text
To provide a traceable, auditable implementation of the Ekādaśī calculation
methodology defined for this project.
```

Every major result should be explainable.

Every major rule should point back to a source.

Every certification should produce evidence.

That is why Pancanga Engine exists: to make a sacred calendrical calculation
transparent enough to be trusted, studied, maintained, and improved without
losing sight of its sources.
