---
title: Type theory
slug: type-theory
page_type: source
category: mathematics
subcategory: foundations
tier: intermediate
keywords:
- types
- lambda calculus
- Martin-Lof
- propositions as types
tags:
- foundations
- logic
author: Wikipedia contributors
date: 2026-05-03
original_source:
  title: Type theory
  project: en.wikipedia.org
  url: https://en.wikipedia.org/wiki/Type_theory
  license: CC BY-SA 4.0
  fetched_at: 2026-05-03T22:20:17Z
  revision_id: W/"1349281143/232f4edf-3ff6-11f1-80c6-7d0061b7e75f/view/html
extraction_status: pending
concepts_generated: []
status: published
---

In [mathematics](https://en.wikipedia.org/wiki/Mathematics "Mathematics"), and [theoretical computer science](https://en.wikipedia.org/wiki/Theoretical_computer_science "Theoretical computer science"), **type theory** is the study of [formal systems](https://en.wikipedia.org/wiki/Formal_system "Formal system") that classify expressions or [mathematical objects](https://en.wikipedia.org/wiki/Mathematical_object "Mathematical object") by their _types_. Roughly speaking, a type plays a similar role to that played by a [data type](https://en.wikipedia.org/wiki/Data_type "Data type") in programming: it specifies what kind of thing an expression is and how it may be used. Type theories are used in the study of [programming languages](https://en.wikipedia.org/wiki/Programming_language "Programming language") ([type systems](https://en.wikipedia.org/wiki/Type_system "Type system")), [formal logic](https://en.wikipedia.org/wiki/Formal_logic "Formal logic"), and the [formalization of mathematics](https://en.wikipedia.org/wiki/Formalization_of_mathematics "Formalization of mathematics").

Some type theories have been proposed as alternatives to [set theory](https://en.wikipedia.org/wiki/Set_theory "Set theory") as a [foundation of mathematics](https://en.wikipedia.org/wiki/Foundations_of_mathematics "Foundations of mathematics"). Examples include [Alonzo Church](https://en.wikipedia.org/wiki/Alonzo_Church "Alonzo Church")'s [simple theory of types](https://en.wikipedia.org/wiki/Simply_typed_lambda_calculus "Simply typed lambda calculus") and [Per Martin-Löf](https://en.wikipedia.org/wiki/Per_Martin-Löf "Per Martin-Löf")'s [intuitionistic type theory](https://en.wikipedia.org/wiki/Intuitionistic_type_theory "Intuitionistic type theory").

Many [proof assistants](https://en.wikipedia.org/wiki/Proof_assistant "Proof assistant") are based on type theory. For example, the underlying formal language of [Rocq](https://en.wikipedia.org/wiki/Rocq "Rocq") (formerly Coq) is the [calculus of inductive constructions](https://en.wikipedia.org/wiki/Calculus_of_inductive_constructions "Calculus of inductive constructions"), while [Lean](https://en.wikipedia.org/wiki/Lean_\(proof_assistant\) "Lean (proof assistant)") is based on [dependent type theory](https://en.wikipedia.org/wiki/Dependent_type_theory "Dependent type theory").

## History

Type theory was created to avoid [paradoxes](https://en.wikipedia.org/wiki/Paradox "Paradox") in [naive set theory](https://en.wikipedia.org/wiki/Naive_set_theory "Naive set theory") and [formal logic](https://en.wikipedia.org/wiki/Formal_logic "Formal logic"), such as [Russell's paradox](https://en.wikipedia.org/wiki/Russell's_paradox "Russell's paradox") which demonstrates that, without proper axioms, it is possible to define the set of all sets that are not members of themselves; this set both contains itself and does not contain itself. Between 1902 and 1908, [Bertrand Russell](https://en.wikipedia.org/wiki/Bertrand_Russell "Bertrand Russell") proposed various solutions to this problem.

By 1908, Russell arrived at a [ramified theory of types](./History_of_type_theory#The_1908_"ramified"_theory_of_types "History of type theory") together with an [axiom of reducibility](https://en.wikipedia.org/wiki/Axiom_of_reducibility "Axiom of reducibility"), both of which appeared in [Whitehead](https://en.wikipedia.org/wiki/Alfred_North_Whitehead "Alfred North Whitehead") and [Russell](https://en.wikipedia.org/wiki/Bertrand_Russell "Bertrand Russell")'s _[Principia Mathematica](https://en.wikipedia.org/wiki/Principia_Mathematica "Principia Mathematica")_ published in 1910, 1912, and 1913. This system avoided contradictions suggested in Russell's paradox by creating a hierarchy of types and then assigning each concrete mathematical entity to a specific type. Entities of a given type were built exclusively of [subtypes](https://en.wikipedia.org/wiki/Subtyping "Subtyping") of that type, thus preventing an entity from being defined using itself. This resolution of Russell's paradox is similar to approaches taken in other formal systems, such as [Zermelo-Fraenkel set theory](https://en.wikipedia.org/wiki/Zermelo–Fraenkel_set_theory "Zermelo–Fraenkel set theory").

Type theory is particularly popular in conjunction with [Alonzo Church](https://en.wikipedia.org/wiki/Alonzo_Church "Alonzo Church")'s [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus "Lambda calculus"). One notable early example of type theory is Church's [simply typed lambda calculus](https://en.wikipedia.org/wiki/Simply_typed_lambda_calculus "Simply typed lambda calculus"). Church's theory of types helped the formal system avoid the [Kleene–Rosser paradox](https://en.wikipedia.org/wiki/Kleene–Rosser_paradox "Kleene–Rosser paradox") that afflicted the original untyped lambda calculus. Church demonstrated that it could serve as a [foundation of mathematics](https://en.wikipedia.org/wiki/Foundations_of_mathematics "Foundations of mathematics") and it was referred to as a [higher-order logic](https://en.wikipedia.org/wiki/Higher-order_logic "Higher-order logic").

In the modern literature, "type theory" refers to a typed system based around lambda calculus. One influential system is [Per Martin-Löf](https://en.wikipedia.org/wiki/Per_Martin-Löf "Per Martin-Löf")'s [intuitionistic type theory](https://en.wikipedia.org/wiki/Intuitionistic_type_theory "Intuitionistic type theory"), which was proposed as a foundation for [constructive mathematics](https://en.wikipedia.org/wiki/Constructivism_\(mathematics\) "Constructivism (mathematics)"). Another is [Thierry Coquand](https://en.wikipedia.org/wiki/Thierry_Coquand "Thierry Coquand")'s [calculus of constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions "Calculus of constructions"), which is used as the foundation by [Rocq](https://en.wikipedia.org/wiki/Rocq_\(software\) "Rocq (software)") (previously known as _Coq_), [Lean](https://en.wikipedia.org/wiki/Lean_\(proof_assistant\) "Lean (proof assistant)"), and other computer [proof assistants](https://en.wikipedia.org/wiki/Proof_assistant "Proof assistant"). Type theory is an active area of research, one direction being the development of [homotopy type theory](/source/homotopy-type-theory/ "Homotopy type theory").

## Applications

### Mathematical foundations

The first computer proof assistant, called [Automath](https://en.wikipedia.org/wiki/Automath "Automath"), used type theory to encode mathematics on a computer. Martin-Löf specifically developed [intuitionistic type theory](https://en.wikipedia.org/wiki/Intuitionistic_type_theory "Intuitionistic type theory") to encode _all_ mathematics to serve as a new foundation for mathematics. There is ongoing research into mathematical foundations using [homotopy type theory](/source/homotopy-type-theory/ "Homotopy type theory").

Mathematicians working in [category theory](/source/category-theory/ "Category theory") already had difficulty working with the widely accepted foundation of [Zermelo–Fraenkel set theory](https://en.wikipedia.org/wiki/Zermelo–Fraenkel_set_theory "Zermelo–Fraenkel set theory"). This led to proposals such as Lawvere's [Elementary Theory of the Category of Sets](https://en.wikipedia.org/wiki/Elementary_Theory_of_the_Category_of_Sets "Elementary Theory of the Category of Sets") (ETCS). Homotopy type theory continues in this line using type theory. Researchers are exploring connections between dependent types (especially the identity type) and [algebraic topology](https://en.wikipedia.org/wiki/Algebraic_topology "Algebraic topology") (specifically [homotopy](/source/homotopy/ "Homotopy")).

### Proof assistants

Much of the current research into type theory is driven by [proof checkers](https://en.wikipedia.org/wiki/Automated_proof_checking "Automated proof checking"), interactive [proof assistants](https://en.wikipedia.org/wiki/Proof_assistant "Proof assistant"), and [automated theorem provers](https://en.wikipedia.org/wiki/Automated_theorem_proving "Automated theorem proving"). Most of these systems use a type theory as the mathematical foundation for encoding proofs, which is not surprising, given the close connection between type theory and programming languages:

*   [LF](https://en.wikipedia.org/wiki/Logical_framework "Logical framework") is used by [Twelf](https://en.wikipedia.org/wiki/Twelf "Twelf"), often to define other type theories;
*   many type theories which fall under [higher-order logic](https://en.wikipedia.org/wiki/Higher-order_logic "Higher-order logic") are used by the [HOL family of provers](https://en.wikipedia.org/wiki/HOL_\(proof_assistant\) "HOL (proof assistant)") and [PVS](https://en.wikipedia.org/wiki/Prototype_Verification_System "Prototype Verification System");
*   computational type theory is used by [NuPRL](https://en.wikipedia.org/wiki/NuPRL "NuPRL");
*   [calculus of constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions "Calculus of constructions") and its derivatives are used by [Rocq](https://en.wikipedia.org/wiki/Rocq_\(software\) "Rocq (software)") (previously known as _Coq_), [Matita](https://en.wikipedia.org/wiki/Matita "Matita"), and [Lean](https://en.wikipedia.org/wiki/Lean_\(proof_assistant\) "Lean (proof assistant)");
*   UTT (Luo's Unified Theory of dependent Types) is used by [Agda](https://en.wikipedia.org/wiki/Agda_\(programming_language\) "Agda (programming language)") which is both a programming language and proof assistant

Many type theories are supported by [LEGO](https://en.wikipedia.org/wiki/LEGO_\(proof_assistant\) "LEGO (proof assistant)") and [Isabelle](https://en.wikipedia.org/wiki/Isabelle_\(proof_assistant\) "Isabelle (proof assistant)"). Isabelle also supports foundations besides type theories, such as [ZFC](https://en.wikipedia.org/wiki/Zermelo–Fraenkel_set_theory "Zermelo–Fraenkel set theory"). [Mizar](https://en.wikipedia.org/wiki/Mizar_system "Mizar system") is an example of a proof system that only supports set theory.

### Programming languages

Any [static program analysis](https://en.wikipedia.org/wiki/Static_program_analysis "Static program analysis"), such as the type checking algorithms in the [semantic analysis](https://en.wikipedia.org/wiki/Semantic_analysis_\(compilers\) "Semantic analysis (compilers)") phase of [compiler](https://en.wikipedia.org/wiki/Compiler "Compiler"), has a connection to type theory. A prime example is [Agda](https://en.wikipedia.org/wiki/Agda_\(programming_language\) "Agda (programming language)"), a programming language which uses UTT (Luo's Unified Theory of dependent Types) for its type system.

The programming language [ML](https://en.wikipedia.org/wiki/ML_\(programming_language\) "ML (programming language)") was developed for manipulating type theories (see _[Logic for Computable Functions](https://en.wikipedia.org/wiki/Logic_for_Computable_Functions "Logic for Computable Functions")_) and its own type system was heavily influenced by them.

### Linguistics

Type theory is also widely used in [formal theories of semantics](https://en.wikipedia.org/wiki/Formal_semantics_\(linguistics\) "Formal semantics (linguistics)") of [natural languages](https://en.wikipedia.org/wiki/Natural_language "Natural language"), especially [Montague grammar](https://en.wikipedia.org/wiki/Montague_grammar "Montague grammar") and its descendants. In particular, [categorial grammars](https://en.wikipedia.org/wiki/Categorial_grammar "Categorial grammar") and [pregroup grammars](https://en.wikipedia.org/wiki/Pregroup_grammar "Pregroup grammar") extensively use type constructors to define the types (_noun_, _verb_, etc.) of words.

The most common construction takes the basic types $e$ and $t$ for individuals and [truth-values](https://en.wikipedia.org/wiki/Truth-value "Truth-value"), respectively, and defines the set of types recursively as follows:

*   if $a$ and $b$ are types, then so is ⁠$\langle a,b\rangle$⁠;
*   nothing except the basic types, and what can be constructed from them by means of the previous clause are types.

A complex type $\langle a,b\rangle$ is the type of [functions](https://en.wikipedia.org/wiki/Function_\(mathematics\) "Function (mathematics)") from entities of type $a$ to entities of type ⁠$b$⁠. Thus one has types like $\langle e,t\rangle$ that are interpreted as elements of the set of functions from entities to truth-values, i.e. [indicator functions](https://en.wikipedia.org/wiki/Indicator_function "Indicator function") of sets of entities. An expression of type $\langle\langle e,t\rangle,t\rangle$ is a function from sets of entities to truth-values, i.e. a (indicator function of a) set of sets. This latter type is standardly taken to be the type of [natural language quantifiers](https://en.wikipedia.org/wiki/Generalized_quantifier "Generalized quantifier"), like _everybody_ or _nobody_ ([Montague](https://en.wikipedia.org/wiki/Richard_Montague "Richard Montague") 1973, [Barwise](https://en.wikipedia.org/wiki/Jon_Barwise "Jon Barwise") and Cooper 1981).

[Type theory with records](https://en.wikipedia.org/wiki/Type_theory_with_records "Type theory with records") is a [formal semantics](https://en.wikipedia.org/wiki/Formal_semantics_\(linguistics\) "Formal semantics (linguistics)") representation framework, using _[records](https://en.wikipedia.org/wiki/Record_\(computer_science\) "Record (computer science)")_ to express _type theory types_. It has been used in [natural language processing](https://en.wikipedia.org/wiki/Natural_language_processing "Natural language processing"), principally [computational semantics](https://en.wikipedia.org/wiki/Computational_semantics "Computational semantics") and [dialogue systems](https://en.wikipedia.org/wiki/Dialogue_systems "Dialogue systems").

### Social sciences

[Gregory Bateson](https://en.wikipedia.org/wiki/Gregory_Bateson "Gregory Bateson") introduced a theory of logical types into the social sciences; his notions of [double bind](https://en.wikipedia.org/wiki/Double_bind "Double bind") and logical levels are based on Russell's theory of types.

## Logic

A type theory is a [mathematical logic](https://en.wikipedia.org/wiki/Mathematical_logic "Mathematical logic"), which is to say it is a collection of [rules of inference](https://en.wikipedia.org/wiki/Rule_of_inference "Rule of inference") that result in [judgments](https://en.wikipedia.org/wiki/Judgment_\(mathematical_logic\) "Judgment (mathematical logic)"). Most logics have judgments asserting "The [proposition](https://en.wikipedia.org/wiki/Proposition "Proposition") $\varphi$ is true", or "The [formula](https://en.wikipedia.org/wiki/Propositional_formula "Propositional formula") $\varphi$ is a [well-formed formula](https://en.wikipedia.org/wiki/Well-formed_formula "Well-formed formula")". A type theory has judgments that define types and assign them to a collection of formal objects, known as terms. A term and its type are often written together as ⁠$\mathrm{term}:\mathsf{type}$⁠.

### Terms

A [term in logic](https://en.wikipedia.org/wiki/Term_\(logic\) "Term (logic)") is [recursively defined](https://en.wikipedia.org/wiki/Recursive_definition "Recursive definition") as a [constant symbol](https://en.wikipedia.org/wiki/Logical_constant "Logical constant"), [variable](https://en.wikipedia.org/wiki/Variable_\(mathematics\) "Variable (mathematics)"), or a [function application](https://en.wikipedia.org/wiki/Function_application "Function application"), where a term is applied to another term. Constant symbols could include the natural number ⁠$0$⁠, the Boolean value ⁠$\texttt{true}$⁠, and functions such as the [successor function](https://en.wikipedia.org/wiki/Successor_function "Successor function") $\mathrm{S}$ and [conditional operator](https://en.wikipedia.org/wiki/Conditional_operator "Conditional operator") ⁠$\mathrm{if}$⁠. Thus some terms could be ⁠$0$⁠, ⁠$(\mathrm{S}\,0)$⁠, ⁠$(\mathrm{S}\,(\mathrm{S}\,0))$⁠, and ⁠$(\mathrm{if}\,\texttt{true}\,0\,(\mathrm{S}\,0))$⁠.

### Judgments

Most type theories have 4 judgments:

*   "$T$ [is a type](/source/type-theory/#Gamma,IsaType)"
*   "$t$ [is a term](/source/type-theory/#Gamma,IsaTerm) of type ⁠$T$⁠"
*   "Type $T_1$ [is equal to](/source/type-theory/#Gamma,IsEq) type ⁠$T_2$⁠"
*   "Terms $t_1$ and $t_2$ both of type $T$ [are equal](/source/type-theory/#Gamma,BothTsAreEq)"

Judgments may follow from assumptions. For example, one might say "assuming $x$ is a term of type $\mathsf{bool}$ and $y$ is a term of type ⁠$\mathsf{nat}$⁠, it follows that $(\textrm{if}\,x\,y\,y)$ is a term of type ⁠$\mathsf{nat}$⁠". Such judgments are formally written with the [turnstile symbol](https://en.wikipedia.org/wiki/Turnstile_\(symbol\) "Turnstile (symbol)") ⁠$\vdash$⁠.

: $x:\mathsf{bool},y:\mathsf{nat}\vdash(\mathrm{if}\,x\,y\,y): \mathsf{nat}$

If there are no assumptions, there will be nothing to the left of the turnstile.

: $\vdash \mathrm{S}:\mathsf{nat}\to\mathsf{nat}$

The list of assumptions on the left is the _context_ of the judgment. Capital greek letters, such as [$\Gamma$](https://en.wikipedia.org/wiki/Gamma "Gamma") and $\Delta$, are common choices to represent some or all of the assumptions. The 4 different judgments are thus usually written as follows.

Formal notation for judgmentsDescription

$\Gamma \vdash T$ Type

$T$ is a type (under assumptions ⁠$\Gamma$⁠).

$\Gamma \vdash t : T$

$t$ is a term of type $T$ (under assumptions ⁠$\Gamma$⁠).

$\Gamma \vdash T_1 = T_2$

Type $T_1$ is equal to type $T_2$ (under assumptions ⁠$\Gamma$⁠).

$\Gamma \vdash t_1 = t_2 : T$

Terms $t_1$ and $t_2$ are both of type $T$ and are equal (under assumptions ⁠$\Gamma$⁠).

Some textbooks use a triple equal sign $\equiv$ to stress that this is [judgmental equality](https://en.wikipedia.org/wiki/Judgment_\(mathematical_logic\) "Judgment (mathematical logic)") and thus an [extrinsic](https://en.wikipedia.org/wiki/Intrinsic_and_extrinsic_properties "Intrinsic and extrinsic properties") notion of equality. The judgments enforce that every term has a type. The type will restrict which rules can be applied to a term.

### Rules of inference

A type theory's [inference rules](https://en.wikipedia.org/wiki/Rule_of_inference "Rule of inference") say what judgments can be made, based on the existence of other judgments. Rules are expressed as a [Gentzen](https://en.wikipedia.org/wiki/Sequent_calculus "Sequent calculus")-style [deduction](https://en.wikipedia.org/wiki/Formal_system "Formal system") using a horizontal line, with the required input judgments above the line and the resulting judgment below the line. For example, the following inference rule states a [substitution](https://en.wikipedia.org/wiki/Substitution_\(logic\) "Substitution (logic)") rule for judgmental equality.$$
\begin{array}{c}
\Gamma\vdash t:T_1 \qquad \Delta\vdash T_1 = T_2 \\
\hline
\Gamma,\Delta\vdash t:T_2
\end{array}
$$The rules are syntactic and work by [rewriting](https://en.wikipedia.org/wiki/Rewriting "Rewriting"). The [metavariables](https://en.wikipedia.org/wiki/Metavariable "Metavariable") ⁠$\Gamma$⁠, ⁠$\Delta$⁠, ⁠$t$⁠, ⁠$T_1$⁠, and ⁠$T_2$⁠ may actually consist of complex terms and types that contain many function applications, not just single symbols.

To generate a particular judgment in type theory, there must be a rule to generate it, as well as rules to generate all of that rule's required inputs, and so on. The applied rules form a [proof tree](https://en.wikipedia.org/wiki/Natural_deduction#Proofs_and_type_theory "Natural deduction"), where the top-most rules need no assumptions. One example of a rule that does not require any inputs is one that states the type of a constant term. For example, to assert that there is a term $0$ of type ⁠$\mathsf{nat}$⁠, one would write the following. $$
\begin{array}{c}
\hline
\vdash 0 : \mathsf{nat} \\
\end{array}
$$

#### Type inhabitation

Generally, the desired conclusion of a proof in type theory is one of [type inhabitation](https://en.wikipedia.org/wiki/Type_inhabitation "Type inhabitation"). The decision problem of type inhabitation (abbreviated by ⁠$\exists t.\Gamma \vdash t : \tau?$⁠) is:

: Given a context ⁠$\Gamma$⁠ and a type ⁠$\tau$⁠, decide whether there exists a term ⁠$t$⁠ that can be assigned the type ⁠$\tau$⁠ in the type environment ⁠$\Gamma$⁠.

[Girard's paradox](https://en.wikipedia.org/wiki/System_U#Girard's_paradox "System U") shows that type inhabitation is strongly related to the [consistency](https://en.wikipedia.org/wiki/Consistency "Consistency") of a type system with Curry–Howard correspondence. To be sound, such a system must have uninhabited types.

A type theory usually has several rules, including ones to:

*   create a judgment (known as a _context_ in this case)
*   add an assumption to the context (context _weakening_)
*   [rearrange the assumptions](https://en.wikipedia.org/wiki/Structural_rule "Structural rule")
*   use an assumption to create a variable
*   define [reflexivity](https://en.wikipedia.org/wiki/Reflexive_relation "Reflexive relation"), [symmetry](https://en.wikipedia.org/wiki/Symmetric_relation "Symmetric relation") and [transitivity](https://en.wikipedia.org/wiki/Transitive_relation "Transitive relation") for judgmental equality
*   define substitution for application of lambda terms
*   list all the interactions of equality, such as substitution
*   define a hierarchy of type universes
*   assert the existence of new types

Also, for each "by rule" type, there are 4 different kinds of rules:

*   "type formation" rules say how to create the type
*   "term introduction" rules define the canonical terms and constructor functions, like "pair" and "S".
*   "term elimination" rules define the other functions like "first", "second", and "R".
*   "computation" rules specify how computation is performed with the type-specific functions.

For examples of rules, an interested reader may follow Appendix A.2 of the _Homotopy Type Theory_ book, or read Martin-Löf's Intuitionistic Type Theory.

## Connections to foundations

The logical framework of a type theory bears a resemblance to [intuitionistic](https://en.wikipedia.org/wiki/Intuitionistic_logic "Intuitionistic logic"), or constructive, logic. Formally, type theory is often cited as an implementation of the [Brouwer–Heyting–Kolmogorov interpretation](https://en.wikipedia.org/wiki/Brouwer–Heyting–Kolmogorov_interpretation "Brouwer–Heyting–Kolmogorov interpretation") of intuitionistic logic. Additionally, connections can be made to [category theory](/source/category-theory/ "Category theory") and [computer programs](https://en.wikipedia.org/wiki/Computer_programming "Computer programming").

### Intuitionistic logic

When used as a foundation, certain types are interpreted to be [propositions](https://en.wikipedia.org/wiki/Propositions "Propositions") (statements that can be proven), and terms inhabiting the type are interpreted to be proofs of that proposition. When some types are interpreted as propositions, there is a set of common types that can be used to connect them to make a [Boolean algebra](https://en.wikipedia.org/wiki/Boolean_algebra_\(structure\) "Boolean algebra (structure)") out of types. However, the logic is not [classical logic](https://en.wikipedia.org/wiki/Classical_logic "Classical logic") but [intuitionistic logic](https://en.wikipedia.org/wiki/Intuitionistic_logic "Intuitionistic logic"), which is to say it does not have the [law of excluded middle](https://en.wikipedia.org/wiki/Law_of_excluded_middle "Law of excluded middle") nor [double negation](https://en.wikipedia.org/wiki/Double_negation "Double negation").

Under this intuitionistic interpretation, there are common types that act as the logical operators:

Logic NameLogic NotationType NotationType Name

True

$\top$

$\top$

Unit Type

False

$\bot$

$\bot$

Empty Type

[Implication](https://en.wikipedia.org/wiki/Material_conditional "Material conditional")

$A \to B$

$A \to B$

Function

[Not](https://en.wikipedia.org/wiki/Not_\(logic\) "Not (logic)")

$\neg A$

$A \to \bot$

Function to Empty Type

[And](https://en.wikipedia.org/wiki/And_\(logic\) "And (logic)")

$A \land B$

$A \times B$

Product Type

[Or](https://en.wikipedia.org/wiki/Or_\(logic\) "Or (logic)")

$A \lor B$

$A + B$

Sum Type

[For All](https://en.wikipedia.org/wiki/Universal_quantification "Universal quantification")

$\forall a \in A, P(a)$

$\Pi a:A.P(a)$

Dependent Product

[Exists](https://en.wikipedia.org/wiki/Existential_quantification "Existential quantification")

$\exists a \in A, P(a)$

$\Sigma a: A.P(a)$

Dependent Sum

Because the law of excluded middle does not hold, there is no term of type ⁠$\Pi A.A+ (A\to\bot)$⁠. Likewise, double negation does not hold, so there is no term of type ⁠$\Pi A.((A\to\bot)\to\bot)\to A$⁠.

It is possible to include the law of excluded middle and double negation into a type theory, by rule or assumption. However, terms may not compute down to canonical terms and it will interfere with the ability to determine if two terms are judgementally equal to each other.

#### Constructive mathematics

Per Martin-Löf proposed his intuitionistic type theory as a foundation for [constructive mathematics](https://en.wikipedia.org/wiki/Constructive_mathematics "Constructive mathematics"). Constructive mathematics requires when proving "there exists an $x$ with property ⁠$P(x)$⁠", one must construct a particular $x$ and a proof that it has property $P$. In type theory, existence is accomplished using the dependent product type, and its proof requires a term of that type.

An example of a non-constructive proof is [proof by contradiction](https://en.wikipedia.org/wiki/Proof_by_contradiction "Proof by contradiction"). The first step is assuming that $x$ does not exist and refuting it by contradiction. The conclusion from that step is "it is not the case that $x$ does not exist". The last step is, by double negation, concluding that $x$ exists. Constructive mathematics does not allow the last step of removing the double negation to conclude that $x$ exists.

Most of the type theories proposed as foundations are constructive, and this includes most of the ones used by proof assistants. It is possible to add non-constructive features to a type theory, by rule or assumption. These include operators on continuations such as [call with current continuation](https://en.wikipedia.org/wiki/Call/cc#Relation_to_non-constructive_logic "Call/cc"). However, these operators tend to break desirable properties such as canonicity and [parametricity](https://en.wikipedia.org/wiki/Parametricity "Parametricity").

### Curry–Howard correspondence

The [Curry–Howard correspondence](https://en.wikipedia.org/wiki/Curry–Howard_correspondence "Curry–Howard correspondence") is the observed similarity between logics and programming languages. The implication in logic, "A $\to$ B" resembles a function from type "A" to type "B". For a variety of logics, the rules are similar to expressions in a programming language's types. The similarity goes farther, as applications of the rules resemble programs in the programming languages. Thus, the correspondence is often summarized as "proofs as programs".

The opposition of terms and types can also be viewed as one of _implementation_ and _specification_. By [program synthesis](https://en.wikipedia.org/wiki/Program_synthesis "Program synthesis"), (the computational counterpart of) type inhabitation can be used to construct (all or parts of) programs from the specification given in the form of type information.

#### Type inference

Many programs that work with type theory (e.g., interactive theorem provers) also do type inferencing. It lets them select the rules that the user intends, with fewer actions by the user.

### Research areas

#### Category theory

Although the initial motivation for [category theory](/source/category-theory/ "Category theory") was far removed from foundationalism, the two fields turned out to have deep connections. As [John Lane Bell](https://en.wikipedia.org/wiki/John_Lane_Bell "John Lane Bell") writes: "In fact categories can _themselves_ be viewed as type theories of a certain kind; this fact alone indicates that type theory is much more closely related to category theory than it is to set theory." In brief, a category can be viewed as a type theory by regarding its objects as types (or _sorts_ ), i.e. "Roughly speaking, a category may be thought of as a type theory shorn of its syntax." A number of significant results follow in this way:

*   [cartesian closed categories](https://en.wikipedia.org/wiki/Cartesian_closed_category "Cartesian closed category") correspond to the typed λ-calculus ([Lambek](https://en.wikipedia.org/wiki/Lambek "Lambek"), 1970);
*   C-monoids (categories with products and exponentials and one non-terminal object) correspond to the untyped λ-calculus (observed independently by Lambek and [Dana Scott](https://en.wikipedia.org/wiki/Dana_Scott "Dana Scott") around 1980);
*   [locally cartesian closed categories](https://en.wikipedia.org/wiki/Locally_cartesian_closed_category "Locally cartesian closed category") correspond to [Martin-Löf type theories](https://en.wikipedia.org/wiki/Martin-Löf_type_theory "Martin-Löf type theory") (Seely, 1984).

The interplay, known as [categorical logic](https://en.wikipedia.org/wiki/Categorical_logic "Categorical logic"), has been a subject of active research since then; see the monograph of Jacobs (1999) for instance.

#### Homotopy type theory

[Homotopy type theory](/source/homotopy-type-theory/ "Homotopy type theory") attempts to combine type theory and category theory. It focuses on equalities, especially equalities between types. [Homotopy type theory](/source/homotopy-type-theory/ "Homotopy type theory") differs from [intuitionistic type theory](https://en.wikipedia.org/wiki/Intuitionistic_type_theory "Intuitionistic type theory") mostly by its handling of the equality type. In 2016, [cubical type theory](https://en.wikipedia.org/wiki/Cubical_type_theory "Cubical type theory") was proposed, which is a homotopy type theory with normalization.

## Definitions

### Terms and types

#### Atomic terms

The most basic types are called atoms, and a term whose type is an atom is known as an atomic term. Common atomic terms included in type theories are [natural numbers](https://en.wikipedia.org/wiki/Natural_number "Natural number"), often notated with the type ⁠$\mathsf{nat}$⁠, [Boolean logic](https://en.wikipedia.org/wiki/Boolean_Logic "Boolean Logic") values (⁠$\texttt{true}$⁠ and ⁠$\texttt{false}$⁠), notated with the type ⁠$\mathsf{bool}$⁠, and [formal variables](https://en.wikipedia.org/wiki/Variable_\(mathematics\) "Variable (mathematics)"), whose type may vary. For example, the following may be atomic terms.

*   $42:\mathsf{nat}$
*   $\texttt{true}:\mathsf{bool}$
*   $x:\mathsf{nat}$
*   $y:\mathsf{bool}$

#### Function terms

In addition to atomic terms, most modern type theories also allow for [functions](https://en.wikipedia.org/wiki/Function_\(mathematics\) "Function (mathematics)"). Function types introduce an arrow symbol, and are [defined inductively](https://en.wikipedia.org/wiki/Recursive_definition "Recursive definition"): If $\sigma$ and $\tau$ are types, then the notation $\sigma\to\tau$ is the type of a function which takes a [parameter](https://en.wikipedia.org/wiki/Parameter "Parameter") of type $\sigma$ and returns a term of type ⁠$\tau$⁠. Types of this form are known as [_simple_ types](https://en.wikipedia.org/wiki/Simply_typed_lambda_calculus "Simply typed lambda calculus").

Some terms may be declared directly as having a simple type, such as the following term, ⁠$\mathrm{add}$⁠, which takes in two natural numbers in sequence and returns one natural number.

: $\mathrm{add}:\mathsf{nat}\to (\mathsf{nat}\to\mathsf{nat})$

Strictly speaking, a simple type only allows for one input and one output, so a more faithful reading of the above type is that $\mathrm{add}$ is a function which takes in a natural number and returns a function of the form ⁠$\mathsf{nat}\to\mathsf{nat}$⁠. The parentheses clarify that $\mathrm{add}$ does not have the type ⁠$(\mathsf{nat}\to \mathsf{nat})\to\mathsf{nat}$⁠, which would be a function which takes in a function of natural numbers and returns a natural number. The convention is that the arrow is [right associative](https://en.wikipedia.org/wiki/Operator_associativity "Operator associativity"), so the parentheses may be dropped from ⁠$\mathrm{add}$⁠'s type.

#### Lambda terms

New function terms may be constructed using [lambda expressions](https://en.wikipedia.org/wiki/Lambda_calculus#Definition "Lambda calculus"), and are called lambda terms. These terms are also defined inductively: a lambda term has the form ⁠$(\lambda v .t)$⁠, where $v$ is a formal variable and $t$ is a term, and its type is notated ⁠$\sigma\to\tau$⁠, where $\sigma$ is the type of ⁠$v$⁠, and $\tau$ is the type of ⁠$t$⁠. The following lambda term represents a function which doubles an input natural number.

: $(\lambda x.\mathrm{add}\,x\,x): \mathsf{nat}\to\mathsf{nat}$

The variable is $x$ and (implicit from the lambda term's type) must have type ⁠$\mathsf{nat}$⁠. The term $\mathrm{add}\,x\,x$ has type ⁠$\mathsf{nat}$⁠, which is seen by applying the function application inference rule twice. Thus, the lambda term has type $\mathsf{nat}\to\mathsf{nat}$, which means it is a function taking a natural number as an [argument](https://en.wikipedia.org/wiki/Argument_of_a_function "Argument of a function") and returning a natural number.

A lambda term is an [anonymous function](https://en.wikipedia.org/wiki/Anonymous_function "Anonymous function") because it lacks a name. The concept of anonymous functions appears in many programming languages.

### Inference Rules

#### Function application

The power of type theories is in specifying how terms may be combined by way of [inference rules](https://en.wikipedia.org/wiki/Rule_of_inference "Rule of inference"). Type theories which have functions also have the inference rule of [function application](https://en.wikipedia.org/wiki/Function_application "Function application"): if $t$ is a term of type ⁠$\sigma\to\tau$⁠, and $s$ is a term of type ⁠$\sigma$⁠, then the application of $t$ to ⁠$s$⁠, often written ⁠$(t\,s)$⁠, has type ⁠$\tau$⁠. For example, if one knows the type notations ⁠$0:\textsf{nat}$⁠, ⁠$1:\textsf{nat}$⁠, and ⁠$2:\textsf{nat}$⁠, then the following type notations can be [deduced](https://en.wikipedia.org/wiki/Deduction_system "Deduction system") from function application.

*   $(\mathrm{add}\,1): \textsf{nat}\to\textsf{nat}$
*   $((\mathrm{add}\,2)\,0): \textsf{nat}$
*   $((\mathrm{add}\,1)((\mathrm{add}\,2)\,0)): \textsf{nat}$

Parentheses indicate the [order of operations](https://en.wikipedia.org/wiki/Order_of_operations "Order of operations"); however, by convention, function application is [left associative](https://en.wikipedia.org/wiki/Left_associative "Left associative"), so parentheses can be dropped where appropriate. In the case of the three examples above, all parentheses could be omitted from the first two, and the third may simplified to ⁠$\mathrm{add}\,1\, (\mathrm{add}\,2\,0): \textsf{nat}$⁠.

#### Reductions

Type theories that allow for lambda terms also include inference rules known as $\beta$-reduction and $\eta$-reduction. They generalize the notion of function application to lambda terms. Symbolically, they are written

*   $(\lambda v. t)\,s\rightarrow t[v \colon= s]$ (⁠$\beta$⁠-reduction).
*   $(\lambda v. t\, v)\rightarrow t$, if $v$ is not a [free variable](https://en.wikipedia.org/wiki/Free_variables_and_bound_variables "Free variables and bound variables") in $t$ (⁠$\eta$⁠-reduction).

The first reduction describes how to evaluate a lambda term: if a lambda expression $(\lambda v .t)$ is applied to a term ⁠$s$⁠, one replaces every occurrence of $v$ in $t$ with ⁠$s$⁠. The second reduction makes explicit the relationship between lambda expressions and function types: if $(\lambda v. t\, v)$ is a lambda term, then it must be that $t$ is a function term because it is being applied to ⁠$v$⁠. Therefore, the lambda expression is equivalent to just ⁠$t$⁠, as both take in one argument and apply $t$ to it.

For example, the following term may be $\beta$-reduced.

: $(\lambda x.\mathrm{add}\,x\,x)\,2\rightarrow \mathrm{add}\,2\,2$

In type theories that also establish notions of [equality](https://en.wikipedia.org/wiki/Equality_\(mathematics\) "Equality (mathematics)") for types and terms, there are corresponding inference rules of $\beta$-equality and $\eta$-equality.

### Common terms and types

#### Empty type

The [empty type](https://en.wikipedia.org/wiki/Empty_type "Empty type") has no terms. The type is usually written $\bot$ or ⁠$\mathbb 0$⁠. One use for the empty type is proofs of [type inhabitation](https://en.wikipedia.org/wiki/Type_inhabitation "Type inhabitation"). If for a type ⁠$a$⁠, it is consistent to derive a function of type ⁠$a\to\bot$⁠, then $a$ is _uninhabited_, which is to say it has no terms.

#### Unit type

The [unit type](https://en.wikipedia.org/wiki/Unit_type "Unit type") has exactly 1 canonical term. The type is written $\top$ or $\mathbb 1$ and the single canonical term is written ⁠$\ast$⁠. The unit type is also used in proofs of type inhabitation. If for a type ⁠$a$⁠, it is consistent to derive a function of type ⁠$\top\to a$⁠, then $a$ is _inhabited_, which is to say it must have one or more terms.

#### Boolean type

The Boolean type has exactly 2 canonical terms. The type is usually written $\textsf{bool}$ or $\mathbb B$ or ⁠$\mathbb 2$⁠. The canonical terms are usually $\mathrm{true}$ and ⁠$\mathrm{false}$⁠.

#### Natural numbers

Natural numbers are usually implemented in the style of [Peano Arithmetic](https://en.wikipedia.org/wiki/Peano_arithmetic "Peano arithmetic"). There is a canonical term $0:\mathsf{nat}$ for zero. Canonical values larger than zero use iterated applications of a [successor function](https://en.wikipedia.org/wiki/Successor_function "Successor function") ⁠$\mathrm{S}:\mathsf{nat}\to\mathsf{nat}$⁠.

### Type constructors

Some type theories allow for types of complex terms, such as functions or lists, to depend on the types of its arguments; these are called [type constructors](https://en.wikipedia.org/wiki/Kind_\(type_theory\) "Kind (type theory)"). For example, a type theory could have the dependent type ⁠$\mathsf{list}\,a$⁠, which should correspond to [lists](https://en.wikipedia.org/wiki/List_\(abstract_data_type\) "List (abstract data type)") of terms, where each term must have type ⁠$a$⁠. In this case, $\mathsf{list}$ has the kind ⁠$U\to U$⁠, where $U$ denotes the [universe](https://en.wikipedia.org/wiki/Universe_\(mathematics\) "Universe (mathematics)") of all types in the theory.

#### Product type

The product type, ⁠$\times$⁠, depends on two types, and its terms are commonly written as [ordered pairs](https://en.wikipedia.org/wiki/Ordered_pair "Ordered pair") ⁠$(s,t)$⁠. The pair $(s,t)$ has the product type ⁠$\sigma\times\tau$⁠, where $\sigma$ is the type of $s$ and $\tau$ is the type of ⁠$t$⁠. Each product type is then usually defined with eliminator functions $\mathrm{first}:\sigma\times\tau\to\sigma$ and ⁠$\mathrm{second}:\sigma\times\tau\to\tau$⁠.

*   $\mathrm{first}\,(s,t)$ returns ⁠$s$⁠, and
*   $\mathrm{second}\,(s,t)$ returns ⁠$t$⁠.

Besides ordered pairs, this type is used for the concepts of [logical conjunction](https://en.wikipedia.org/wiki/Logical_conjunction "Logical conjunction") and [intersection](https://en.wikipedia.org/wiki/Intersection "Intersection").

#### Sum type

The sum type is written as either $+$ or ⁠$\sqcup$⁠. In programming languages, sum types may be referred to as [tagged unions](https://en.wikipedia.org/wiki/Tagged_union "Tagged union"). Each type $\sigma\sqcup\tau$ is usually defined with [constructors](https://en.wikipedia.org/wiki/Constructor_\(programming\) "Constructor (programming)") $\mathrm{left}:\sigma\to(\sigma\sqcup\tau)$ and ⁠$\mathrm{right}:\tau\to(\sigma\sqcup\tau)$⁠, which are [injective](https://en.wikipedia.org/wiki/Injective_function "Injective function"), and an eliminator function $\mathrm{match}:(\sigma\to\rho)\to(\tau\to\rho)\to(\sigma\sqcup\tau)\to\rho$ such that

*   $\mathrm{match}\,f\,g\,(\mathrm{left}\,x)$ returns ⁠$f\,x$⁠, and
*   $\mathrm{match}\,f\,g\,(\mathrm{right}\,y)$ returns ⁠$g\,y$⁠.

The sum type is used for the concepts of [logical disjunction](https://en.wikipedia.org/wiki/Logical_or "Logical or") and [union](https://en.wikipedia.org/wiki/Union_\(set_theory\) "Union (set theory)").

### Polymorphic types

Some theories also allow terms to have their definitions depend on types. For instance, an identity function of any type could be written as ⁠$\lambda x.x:\forall\alpha. \alpha\to\alpha$⁠. The function is said to be polymorphic in ⁠$\alpha$⁠, or generic in ⁠$x$⁠.

As another example, consider a function ⁠$\mathrm{append}$⁠, which takes in a $\mathsf{list}\,a$ and a term of type ⁠$a$⁠, and returns the list with the element at the end. The type annotation of such a function would be ⁠$\mathrm{append}:\forall\,a.\mathsf{list}\,a\to a\to\mathsf{list}\,a$⁠, which can be read as "for any type ⁠$a$⁠, pass in a $\mathsf{list}\,a$ and an ⁠$a$⁠, and return a ⁠$\mathsf{list}\,a$⁠". Here $\mathrm{append}$ is polymorphic in ⁠$a$⁠.

#### Products and sums

With polymorphism, the eliminator functions can be defined generically for _all_ product types as $\mathrm{first}:\forall\,\sigma\,\tau.\sigma\times\tau\to\sigma$ and ⁠$\mathrm{second}:\forall\,\sigma\,\tau.\sigma\times\tau\to\tau$⁠.

*   $\mathrm{first}\,(s,t)$ returns ⁠$s$⁠, and
*   $\mathrm{second}\,(s,t)$ returns ⁠$t$⁠.

Likewise, the sum type constructors can be defined for all valid types of sum members as $\mathrm{left}:\forall\,\sigma\,\tau.\sigma\to(\sigma\sqcup\tau)$ and ⁠$\mathrm{right}:\forall\,\sigma\,\tau.\tau\to(\sigma\sqcup\tau)$⁠, which are [injective](https://en.wikipedia.org/wiki/Injective_function "Injective function"), and the eliminator function can be given as $\mathrm{match}:\forall\,\sigma\,\tau\,\rho.(\sigma\to\rho)\to(\tau\to\rho)\to(\sigma\sqcup\tau)\to\rho$ such that

*   $\mathrm{match}\,f\,g\,(\mathrm{left}\,x)$ returns ⁠$f\,x$⁠, and
*   $\mathrm{match}\,f\,g\,(\mathrm{right}\,y)$ returns ⁠$g\,y$⁠.

### Dependent typing

Some theories also permit types to be dependent on terms instead of types. For example, a theory could have the type ⁠$\mathsf{vector}\,n$⁠, where $n$ is a term of type $\mathsf{nat}$ encoding the length of the [vector](https://en.wikipedia.org/wiki/Vector_space "Vector space"). This allows for greater specificity and [type safety](https://en.wikipedia.org/wiki/Type_safety "Type safety"): functions with vector length restrictions or length matching requirements, such as the [dot product](https://en.wikipedia.org/wiki/Dot_product "Dot product"), can encode this requirement as part of the type.

There are foundational issues that can arise from dependent types if a theory is not careful about what dependences are allowed, such as [Girard's Paradox](https://en.wikipedia.org/wiki/Girard's_paradox "Girard's paradox"). The logician [Henk Barendegt](https://en.wikipedia.org/wiki/Henk_Barendregt "Henk Barendregt") introduced the [lambda cube](https://en.wikipedia.org/wiki/Lambda_cube "Lambda cube") as a framework for studying various restrictions and levels of dependent typing.

#### Dependent products and sums

Two common [type dependences](https://en.wikipedia.org/wiki/Dependent_type "Dependent type"), dependent product and dependent sum types, allow for the theory to encode [BHK intuitionistic logic](https://en.wikipedia.org/wiki/BHK_interpretation "BHK interpretation") by acting as equivalents to [universal and existential quantification](https://en.wikipedia.org/wiki/Quantification_\(logic\) "Quantification (logic)"); this is formalized by [Curry–Howard correspondence](https://en.wikipedia.org/wiki/Curry–Howard_correspondence "Curry–Howard correspondence"). As they also connect to [products](https://en.wikipedia.org/wiki/Cartesian_product "Cartesian product") and [sums](https://en.wikipedia.org/wiki/Disjoint_union "Disjoint union") in [set theory](https://en.wikipedia.org/wiki/Set_theory "Set theory"), they are often written with the symbols $\Pi$ and ⁠$\Sigma$⁠, respectively.

Sum types are seen in [dependent pairs](https://en.wikipedia.org/wiki/Dependent_type "Dependent type"), where the second type depends on the value of the first term. This arises naturally in computer science where functions may return different types of outputs based on the input. For example, the Boolean type is usually defined with an eliminator function ⁠$\mathrm{if}$⁠, which takes three arguments and behaves as follows.

*   $\mathrm{if}\,\texttt{true}\,x\,y$ returns ⁠$x$⁠, and
*   $\mathrm{if}\,\texttt{false}\,x\,y$ returns ⁠$y$⁠.

Ordinary definitions of $\mathrm{if}$ require $x$ and $y$ to have the same type. If the type theory allows for dependent types, then it is possible to define a dependent type $x:\mathsf{bool}\,\vdash\,\mathrm{TF}\,x:U\to U\to U$ such that

*   $\mathrm{TF}\,\texttt{true}\,\sigma\,\tau$ returns ⁠$\sigma$⁠, and
*   $\mathrm{TF}\,\texttt{false}\,\sigma\,\tau$ returns ⁠$\tau$⁠.

The type of $\mathrm{if}$ may then be written as ⁠$\forall\,\sigma\,\tau.\Pi_{x:\mathsf{bool} }.\sigma\to\tau\to\mathrm{TF}\,x\,\sigma\,\tau$⁠.

#### Identity type

Following the notion of Curry–Howard Correspondence, the [identity type](https://en.wikipedia.org/wiki/Identity_type "Identity type") is a type introduced to mirror [propositional equivalence](https://en.wikipedia.org/wiki/Propositional_logic "Propositional logic"), as opposed to the [judgmental (syntactic) equivalence](https://en.wikipedia.org/wiki/Judgment_\(mathematical_logic\) "Judgment (mathematical logic)") that type theory already provides.

An identity type requires two terms of the same type and is written with the symbol ⁠$=$⁠. For example, if $x+1$ and $1+x$ are terms, then $x+1=1+x$ is a possible type. Canonical terms are created with a reflexivity function, ⁠$\mathrm{refl}$⁠. For a term ⁠$t$⁠, the call $\mathrm{refl}\,t$ returns the canonical term inhabiting the type ⁠$t=t$⁠.

The complexities of equality in type theory make it an active research topic; [homotopy type theory](/source/homotopy-type-theory/ "Homotopy type theory") is a notable area of research that mainly deals with equality in type theory.

#### Inductive types

Inductive types are a general template for creating a large variety of types. In fact, all the types described above and more can be defined using the rules of inductive types. Two methods of generating inductive types are [induction-recursion](https://en.wikipedia.org/wiki/Induction-recursion "Induction-recursion") and [induction-induction](https://en.wikipedia.org/wiki/Induction-induction "Induction-induction"). A method that only uses lambda terms is [Scott encoding](https://en.wikipedia.org/wiki/Mogensen–Scott_encoding "Mogensen–Scott encoding").

Some [proof assistants](https://en.wikipedia.org/wiki/Proof_assistant "Proof assistant"), such as [Rocq](https://en.wikipedia.org/wiki/Rocq_\(software\) "Rocq (software)") (previously known as _Coq_) and [Lean](https://en.wikipedia.org/wiki/Lean_\(proof_assistant\) "Lean (proof assistant)"), are based on the calculus for inductive constructions, which is a [calculus of constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions "Calculus of constructions") with inductive types.

## Differences from set theory

The most commonly accepted [foundation for mathematics](https://en.wikipedia.org/wiki/Foundations_of_mathematics "Foundations of mathematics") is [first-order logic](https://en.wikipedia.org/wiki/First-order_logic "First-order logic") with the [language](https://en.wikipedia.org/wiki/Formal_language "Formal language") and [axioms](https://en.wikipedia.org/wiki/Axiom "Axiom") of [Zermelo–Fraenkel set theory](https://en.wikipedia.org/wiki/Zermelo–Fraenkel_set_theory "Zermelo–Fraenkel set theory") with the [axiom of choice](https://en.wikipedia.org/wiki/Axiom_of_choice "Axiom of choice"), abbreviated ZFC. Type theories having sufficient [expressibility](https://en.wikipedia.org/wiki/Expressive_power_\(computer_science\) "Expressive power (computer science)") may also act as a foundation of mathematics. There are a number of differences between these two approaches.

*   Set theory has both [rules](https://en.wikipedia.org/wiki/Rule_of_inference "Rule of inference") and [axioms](https://en.wikipedia.org/wiki/Axiom "Axiom"), while type theories only have rules. Type theories, in general, do not have axioms and are defined by their rules of inference.
*   Classical set theory and logic have the [law of excluded middle](https://en.wikipedia.org/wiki/Law_of_excluded_middle "Law of excluded middle"). When a type theory encodes the concepts of "and" and "or" as types, it leads to [intuitionistic logic](https://en.wikipedia.org/wiki/Intuitionistic_logic "Intuitionistic logic"), and does not necessarily have the law of excluded middle.
*   In set theory, an element is not restricted to one set. The element can appear in subsets and unions with other sets. In type theory, terms (generally) belong to only one type. Where a subset would be used, type theory can use a [predicate function](https://en.wikipedia.org/wiki/Predicate_\(mathematical_logic\) "Predicate (mathematical logic)") or use a dependently-typed product type, where each element $x$ is paired with a proof that the subset's property holds for ⁠$x$⁠. Where a union would be used, type theory uses the sum type, which contains new canonical terms.
*   Type theory has a built-in notion of computation. Thus, "1+1" and "2" are different terms in type theory, but they compute to the same value. Moreover, functions are defined computationally as lambda terms. In set theory, "1+1=2" means that "1+1" is just another way to refer the value "2". Type theory's computation does require a complicated concept of equality.
*   Set theory [encodes numbers as sets](https://en.wikipedia.org/wiki/Set-theoretic_definition_of_natural_numbers "Set-theoretic definition of natural numbers"). Type theory can encode numbers as functions using [Church encoding](https://en.wikipedia.org/wiki/Church_encoding "Church encoding"), or more naturally as [inductive types](https://en.wikipedia.org/wiki/Intuitionistic_type_theory#Inductive_types "Intuitionistic type theory"), and the construction closely resembles [Peano's axioms](https://en.wikipedia.org/wiki/Peano_axioms "Peano axioms").
*   In type theory, proofs have types whereas in set theory, proofs are part of the underlying first-order logic.

Proponents of type theory will also point out its connection to constructive mathematics through the [BHK interpretation](https://en.wikipedia.org/wiki/BHK_interpretation "BHK interpretation"), its connection to logic by the [Curry–Howard isomorphism](https://en.wikipedia.org/wiki/Curry–Howard_isomorphism "Curry–Howard isomorphism"), and its connections to [category theory](/source/category-theory/ "Category theory").

### Properties of type theories

Terms usually belong to a single type. However, there are type theories that define "subtyping".

Computation takes place by repeated application of rules. Many types of theories are [strongly normalizing](https://en.wikipedia.org/wiki/Strongly_normalizing "Strongly normalizing"), which means that any order of applying the rules will always end in the same result. However, some are not. In a normalizing type theory, the one-directional computation rules are called "reduction rules", and applying the rules "reduces" the term. If a rule is not one-directional, it is called a "conversion rule".

Some combinations of types are equivalent to other combinations of types. When functions are considered "exponentiation", the combinations of types can be written similarly to algebraic identities. Thus, ⁠${\mathbb 0} + A \cong A$⁠, ⁠${\mathbb 1} \times A \cong A$⁠, ⁠${\mathbb 1} + {\mathbb 1} \cong {\mathbb 2}$⁠, ⁠$A^{B+C} \cong A^B \times A^C$⁠, ⁠$A^{B\times C} \cong (A^B)^C$⁠.

### Axioms

Most type theories do not have [axioms](https://en.wikipedia.org/wiki/Axiom "Axiom"). This is because a type theory is defined by its rules of inference. This is a source of confusion for people familiar with Set Theory, where a theory is defined by both the rules of inference for a logic (such as [first-order logic](https://en.wikipedia.org/wiki/First-order_logic "First-order logic")) and axioms about sets.

Sometimes, a type theory will add a few axioms. An axiom is a judgment that is accepted without a derivation using the rules of inference. They are often added to ensure properties that cannot be added cleanly through the rules.

Axioms can cause problems if they introduce terms without a way to compute on those terms. That is, axioms can interfere with the [normalizing property](https://en.wikipedia.org/wiki/Normal_form_\(abstract_rewriting\) "Normal form (abstract rewriting)") of the type theory.

Some commonly encountered axioms are:

*   "Axiom K" ensures "uniqueness of identity proofs". That is, that every term of an identity type is equal to reflexivity.
*   "Univalence axiom" holds that equivalence of types is equality of types. The research into this property led to [cubical type theory](https://en.wikipedia.org/wiki/Cubical_type_theory "Cubical type theory"), where the property holds without needing an axiom.
*   "Law of excluded middle" is often added to satisfy users who want [classical logic](https://en.wikipedia.org/wiki/Classical_logic "Classical logic"), instead of intuitionistic logic.

The [axiom of choice](https://en.wikipedia.org/wiki/Axiom_of_choice "Axiom of choice") does not need to be added to type theory, because in most type theories it can be derived from the rules of inference. This is because of the [constructive](https://en.wikipedia.org/wiki/Constructive_mathematics "Constructive mathematics") nature of type theory, where proving that a value exists requires a method to compute the value. The axiom of choice is less powerful in type theory than most set theories, because type theory's functions must be computable and, being syntax-driven, the number of terms in a type must be countable. (See _[Axiom of choice § In constructive mathematics](https://en.wikipedia.org/wiki/Axiom_of_choice#In_constructive_mathematics "Axiom of choice")_.)

## List of type theories

### Major

*   [Simply typed lambda calculus](https://en.wikipedia.org/wiki/Simply_typed_lambda_calculus "Simply typed lambda calculus") which is a [higher-order logic](https://en.wikipedia.org/wiki/Higher-order_logic "Higher-order logic")
*   [Intuitionistic type theory](https://en.wikipedia.org/wiki/Intuitionistic_type_theory "Intuitionistic type theory")
*   [System F](https://en.wikipedia.org/wiki/System_F "System F")
*   [LF](https://en.wikipedia.org/wiki/Logical_framework "Logical framework") is often used to define other type theories
*   [Calculus of constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions "Calculus of constructions") and its derivatives

### Minor

*   [Automath](https://en.wikipedia.org/wiki/Automath "Automath")
*   [ST type theory](https://en.wikipedia.org/wiki/ST_type_theory "ST type theory")
*   UTT (Luo's unified theory of dependent types)
*   some forms of [combinatory logic](https://en.wikipedia.org/wiki/Combinatory_logic "Combinatory logic")
*   others defined in the [lambda cube](https://en.wikipedia.org/wiki/Lambda_cube "Lambda cube") (also known as [pure type systems](https://en.wikipedia.org/wiki/Pure_type_system "Pure type system"))
*   others under the name [typed lambda calculus](https://en.wikipedia.org/wiki/Typed_lambda_calculus "Typed lambda calculus")

### Active research

*   [Homotopy type theory](/source/homotopy-type-theory/ "Homotopy type theory") explores equality of types
*   [Cubical type theory](https://ncatlab.org/nlab/show/cubical+type+theory "nlab:cubical+type+theory") is an implementation of homotopy type theory
