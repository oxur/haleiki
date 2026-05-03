---
title: Algebraic structure
slug: algebraic-structure
page_type: source
category: mathematics
subcategory: algebra
tier: foundational
keywords:
- operations
- axioms
- set with operations
tags:
- algebra
- structure
author: Wikipedia contributors
date: 2026-05-03
original_source:
  title: Algebraic structure
  project: en.wikipedia.org
  url: https://en.wikipedia.org/wiki/Algebraic_structure
  license: CC BY-SA 4.0
  fetched_at: 2026-05-03T22:20:16Z
  revision_id: W/"1348266671/581e69ca-3cbf-11f1-bb1d-0b09208a070c/view/html
extraction_status: pending
concepts_generated: []
status: published
---

In [mathematics](https://en.wikipedia.org/wiki/Mathematics "Mathematics"), an **algebraic structure** or **algebraic system** consists of a nonempty [set](https://en.wikipedia.org/wiki/Set_\(mathematics\) "Set (mathematics)") _A_ (called the **underlying set**, **carrier set** or **domain**), a collection of [operations](https://en.wikipedia.org/wiki/Operation_\(mathematics\) "Operation (mathematics)") on _A_ (typically [binary operations](https://en.wikipedia.org/wiki/Binary_operation "Binary operation") such as addition and multiplication), and a finite set of [identities](https://en.wikipedia.org/wiki/Identity_\(mathematics\) "Identity (mathematics)") (known as [_axioms_](https://en.wikipedia.org/wiki/Axiom#Non-logical_axioms "Axiom")) that these operations must satisfy.

An algebraic structure may be based on other algebraic structures with operations and axioms involving several structures. For instance, a [vector space](https://en.wikipedia.org/wiki/Vector_space "Vector space") involves a second structure called a [field](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)"), and an operation called _scalar multiplication_ between elements of the field (called _[scalars](https://en.wikipedia.org/wiki/Scalar_\(mathematics\) "Scalar (mathematics)")_), and elements of the vector space (called _[vectors](https://en.wikipedia.org/wiki/Vector_\(mathematics_and_physics\) "Vector (mathematics and physics)")_).

[Abstract algebra](/source/abstract-algebra/ "Abstract algebra") is the name that is commonly given to the study of algebraic structures. The general theory of algebraic structures has been formalized in [universal algebra](https://en.wikipedia.org/wiki/Universal_algebra "Universal algebra"). [Category theory](/source/category-theory/ "Category theory") is another formalization that includes other [mathematical structures](https://en.wikipedia.org/wiki/Mathematical_structure "Mathematical structure") and [functions](https://en.wikipedia.org/wiki/Function_\(mathematics\) "Function (mathematics)") between structures of the same type ([homomorphisms](https://en.wikipedia.org/wiki/Homomorphism "Homomorphism")).

In universal algebra, an algebraic structure is called an _algebra_; this term may be ambiguous, since, in other contexts, [an algebra](https://en.wikipedia.org/wiki/An_algebra "An algebra") is an algebraic structure that is a vector space over a [field](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)") or a [module](https://en.wikipedia.org/wiki/Module_\(ring_theory\) "Module (ring theory)") over a [commutative ring](https://en.wikipedia.org/wiki/Commutative_ring "Commutative ring").

The collection of all structures of a given type (same operations and same laws) is called a [variety](https://en.wikipedia.org/wiki/Variety_\(universal_algebra\) "Variety (universal algebra)") in universal algebra; this term is also used with a completely different meaning in [algebraic geometry](https://en.wikipedia.org/wiki/Algebraic_geometry "Algebraic geometry"), as an abbreviation of [algebraic variety](https://en.wikipedia.org/wiki/Algebraic_variety "Algebraic variety"). In category theory, the collection of all structures of a given type and homomorphisms between them form a [concrete category](https://en.wikipedia.org/wiki/Concrete_category "Concrete category").

## Introduction

[Addition](https://en.wikipedia.org/wiki/Addition "Addition") and [multiplication](https://en.wikipedia.org/wiki/Multiplication "Multiplication") are prototypical examples of [operations](https://en.wikipedia.org/wiki/Operation_\(mathematics\) "Operation (mathematics)") that combine two elements of a set to produce a third element of the same set. These operations obey several algebraic laws. For example, _a_ + (_b_ + _c_) = (_a_ + _b_) + _c_ and _a_(_bc_) = (_ab_)_c_ are [associative laws](https://en.wikipedia.org/wiki/Associative_law "Associative law"), and _a_ + _b_ = _b_ + _a_ and _ab_ = _ba_ are [commutative laws](https://en.wikipedia.org/wiki/Commutative_law "Commutative law"). Many systems studied by mathematicians have operations that obey some, but not necessarily all, of the laws of ordinary arithmetic. For example, the possible moves of an object in three-dimensional space can be combined by performing a first move of the object, and then a second move from its new position. Such moves, formally called [rigid motions](https://en.wikipedia.org/wiki/Rigid_motion "Rigid motion"), obey the associative law, but fail to satisfy the commutative law.

Sets with one or more operations that obey specific laws are called _algebraic structures_. When a new problem involves the same laws as such an algebraic structure, all the results that have been proved using only the laws of the structure can be directly applied to the new problem.

In full generality, algebraic structures may involve an arbitrary collection of operations, including operations that combine more than two elements (higher [arity](https://en.wikipedia.org/wiki/Arity "Arity") operations) and operations that take only one [argument](https://en.wikipedia.org/wiki/Argument_of_a_function "Argument of a function") ([unary operations](https://en.wikipedia.org/wiki/Unary_operation "Unary operation")) or even zero arguments ([nullary operations](https://en.wikipedia.org/wiki/Nullary_operation "Nullary operation")). The examples listed below are by no means a complete list, but include the most common structures taught in undergraduate courses.

## Common axioms

### Equational axioms

An axiom of an algebraic structure often has the form of an [identity](https://en.wikipedia.org/wiki/Identity_\(mathematics\) "Identity (mathematics)"), that is, an [equation](https://en.wikipedia.org/wiki/Equation_\(mathematics\) "Equation (mathematics)") such that the two sides of the [equals sign](https://en.wikipedia.org/wiki/Equals_sign "Equals sign") are [expressions](https://en.wikipedia.org/wiki/Expression_\(mathematics\) "Expression (mathematics)") that involve operations of the algebraic structure and [variables](https://en.wikipedia.org/wiki/Variable_\(mathematics\) "Variable (mathematics)"). If the variables in the identity are replaced by arbitrary elements of the algebraic structure, the equality must remain true. Here are some common examples.

\*\*Commutativity\*\* : An operation $*$ is commutative if $$
x*y=y*x
$$ for every x and y in the algebraic structure. \*\*Associativity\*\* : An operation $*$ is associative if $$
(x*y)*z=x*(y*z)
$$ for every x, y and z in the algebraic structure. \*\*Left distributivity\*\* : An operation $*$ is left-distributive with respect to another operation $+$ if $$
x*(y+z)=(x*y)+(x*z)
$$ for every x, y and z in the algebraic structure (the second operation is denoted here as $+$, because the second operation is addition in many common examples). \*\*Right distributivity\*\* : An operation $*$ is right-distributive with respect to another operation $+$ if $$
(y+z)*x=(y*x)+(z*x)
$$ for every x, y and z in the algebraic structure. \*\*Distributivity\*\* : An operation $*$ is distributive with respect to another operation $+$ if it is both left-distributive and right-distributive. If the operation $*$ is commutative, left and right distributivity are both equivalent to distributivity.

### Existential axioms

Some common axioms contain an [existential clause](https://en.wikipedia.org/wiki/Existential_clause "Existential clause"). In general, such a clause can be avoided by introducing further operations, and replacing the existential clause by an identity involving the new operation. More precisely, let us consider an axiom of the form _"for all X there is y such that_ $f(X,y)=g(X,y)$", where X is a k-[tuple](https://en.wikipedia.org/wiki/Tuple "Tuple") of variables. Choosing a specific value of y for each value of X defines a function $\varphi:X\mapsto y,$ which can be viewed as an operation of [arity](https://en.wikipedia.org/wiki/Arity "Arity") k, and the axiom becomes the identity $f(X,\varphi(X))=g(X,\varphi(X)).$

The introduction of such auxiliary operation complicates slightly the statement of an axiom, but has some advantages. Given a specific algebraic structure, the proof that an existential axiom is satisfied consists generally of the definition of the auxiliary function, completed with straightforward verifications. Also, when computing in an algebraic structure, one generally uses explicitly the auxiliary operations. For example, in the case of [numbers](https://en.wikipedia.org/wiki/Number "Number"), the [additive inverse](https://en.wikipedia.org/wiki/Additive_inverse "Additive inverse") is provided by the unary minus operation $x\mapsto -x.$

Also, in [universal algebra](https://en.wikipedia.org/wiki/Universal_algebra "Universal algebra"), a [variety](https://en.wikipedia.org/wiki/Variety_\(universal_algebra\) "Variety (universal algebra)") is a class of algebraic structures that share the same operations, and the same axioms, with the condition that all axioms are identities. What precedes shows that existential axioms of the above form are accepted in the definition of a variety.

Here are some of the most common existential axioms.

\*\*Identity element\*\* : A binary operation $*$ has an identity element if there is an element e such that $$
x*e=x\quad \text{and} \quad e*x=x
$$ for all x in the structure. Here, the auxiliary operation is the operation of arity zero that has e as its result. \*\*Inverse element\*\* : Given a binary operation $*$ that has an identity element e, an element x is invertible if it has an inverse element, that is, if there exists an element $\operatorname{inv}(x)$ such that $$
\operatorname{inv}(x)*x=e \quad \text{and} \quad x*\operatorname{inv}(x)=e.
$$For example, a group is an algebraic structure with a binary operation that is associative, has an identity element, and for which all elements are invertible.

### Non-equational axioms

The axioms of an algebraic structure can be any [first-order formula](https://en.wikipedia.org/wiki/First-order_logic "First-order logic"), that is a formula involving [logical connectives](https://en.wikipedia.org/wiki/Logical_connective "Logical connective") (such as _"and"_, _"or"_ and _"not"_), and [logical quantifiers](https://en.wikipedia.org/wiki/Logical_quantifier "Logical quantifier") ($\forall, \exists$) that apply to elements (not to subsets) of the structure.

Such a typical axiom is inversion in [fields](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)"). This axiom cannot be reduced to axioms of preceding types. (it follows that fields do not form a [variety](https://en.wikipedia.org/wiki/Variety_\(universal_algebra\) "Variety (universal algebra)") in the sense of [universal algebra](https://en.wikipedia.org/wiki/Universal_algebra "Universal algebra").) It can be stated: _"Every nonzero element of a field is [invertible](https://en.wikipedia.org/wiki/Invertible_element "Invertible element");"_ or, equivalently: _the structure has a [unary operation](https://en.wikipedia.org/wiki/Unary_operation "Unary operation") inv_ such that

: $\forall x, \quad x=0 \quad\text{or} \quad x \cdot\operatorname{inv}(x)=1.$

The operation inv can be viewed either as a [partial operation](https://en.wikipedia.org/wiki/Partial_operation "Partial operation") that is not defined for _x_ = 0; or as an ordinary function whose value at 0 is arbitrary and must not be used.

## Common algebraic structures

### One set with operations

**Simple structures**: **no** [binary operation](https://en.wikipedia.org/wiki/Binary_operation "Binary operation"):

*   [Set](https://en.wikipedia.org/wiki/Set_\(mathematics\) "Set (mathematics)"): a degenerate algebraic structure _S_ having no operations.

**Group-like structures**: **one** binary operation. The binary operation can be indicated by any symbol, or with no symbol (juxtaposition) as is done for ordinary multiplication of real numbers.

*   [Group](/source/group-mathematics/ "Group (mathematics)"): a [monoid](https://en.wikipedia.org/wiki/Monoid "Monoid") with a unary operation (inverse), giving rise to [inverse elements](https://en.wikipedia.org/wiki/Inverse_element "Inverse element").
*   [Abelian group](https://en.wikipedia.org/wiki/Abelian_group "Abelian group"): a group whose binary operation is [commutative](https://en.wikipedia.org/wiki/Commutative "Commutative").

**Ring-like structures** or **Ringoids**: **two** binary operations, often called [addition](https://en.wikipedia.org/wiki/Addition "Addition") and [multiplication](https://en.wikipedia.org/wiki/Multiplication "Multiplication"), with multiplication [distributing](https://en.wikipedia.org/wiki/Distributivity "Distributivity") over addition.

*   [Ring](https://en.wikipedia.org/wiki/Ring_\(mathematics\) "Ring (mathematics)"): a semiring whose additive monoid is an abelian group.
*   [Division ring](https://en.wikipedia.org/wiki/Division_ring "Division ring"): a [nontrivial](https://en.wikipedia.org/wiki/Zero_ring "Zero ring") ring in which [division](https://en.wikipedia.org/wiki/Division_\(mathematics\) "Division (mathematics)") by nonzero elements is defined.
*   [Commutative ring](https://en.wikipedia.org/wiki/Commutative_ring "Commutative ring"): a ring in which the multiplication operation is commutative.
*   [Field](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)"): a commutative division ring (i.e. a commutative ring which contains a multiplicative inverse for every nonzero element).

**Lattice structures**: **two** or more binary operations, including operations called [meet and join](https://en.wikipedia.org/wiki/Meet_and_join "Meet and join"), connected by the [absorption law](https://en.wikipedia.org/wiki/Absorption_law "Absorption law").

*   [Complete lattice](https://en.wikipedia.org/wiki/Complete_lattice "Complete lattice"): a lattice in which arbitrary [meet and joins](https://en.wikipedia.org/wiki/Meet_and_join "Meet and join") exist.
*   [Bounded lattice](https://en.wikipedia.org/wiki/Bounded_lattice "Bounded lattice"): a lattice with a [greatest element](https://en.wikipedia.org/wiki/Greatest_element "Greatest element") and least element.
*   [Distributive lattice](https://en.wikipedia.org/wiki/Distributive_lattice "Distributive lattice"): a lattice in which each of meet and join [distributes](https://en.wikipedia.org/wiki/Distributive_lattice "Distributive lattice") over the other. A [power set](https://en.wikipedia.org/wiki/Power_set "Power set") under union and intersection forms a distributive lattice.
*   [Boolean algebra](https://en.wikipedia.org/wiki/Boolean_algebra_\(structure\) "Boolean algebra (structure)"): a complemented distributive lattice. Either of meet or join can be defined in terms of the other and complementation.

### Two sets with operations

*   [Module](https://en.wikipedia.org/wiki/Module_\(mathematics\) "Module (mathematics)"): an abelian group _M_ and a ring _R_ acting as operators on _M_. The members of _R_ are sometimes called [scalars](https://en.wikipedia.org/wiki/Scalar_\(mathematics\) "Scalar (mathematics)"), and the binary operation of _scalar multiplication_ is a function _R_ × _M_ → _M_, which satisfies several axioms. Counting the ring operations these systems have at least three operations.
*   [Vector space](https://en.wikipedia.org/wiki/Vector_space "Vector space"): a module where the ring _R_ is a [field](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)") or, in some contexts, a [division ring](https://en.wikipedia.org/wiki/Division_ring "Division ring").

*   [Algebra over a field](https://en.wikipedia.org/wiki/Algebra_over_a_field "Algebra over a field"): a module over a field, which also carries a multiplication operation that is compatible with the module structure. This includes distributivity over addition and [linearity](https://en.wikipedia.org/wiki/Bilinear_map "Bilinear map") with respect to multiplication.
*   [Inner product space](https://en.wikipedia.org/wiki/Inner_product_space "Inner product space"): a field _F_ and vector space _V_ with a [definite bilinear form](https://en.wikipedia.org/wiki/Definite_bilinear_form "Definite bilinear form")_V_ × _V_ → _F_.

## Hybrid structures

Algebraic structures can also coexist with added structure of non-algebraic nature, such as [partial order](https://en.wikipedia.org/wiki/Partially_ordered_set#Formal_definition "Partially ordered set") or a [topology](https://en.wikipedia.org/wiki/Topology "Topology"). The added structure must be compatible, in some sense, with the algebraic structure.

*   [Topological group](https://en.wikipedia.org/wiki/Topological_group "Topological group"): a group with a topology compatible with the group operation.
*   [Lie group](https://en.wikipedia.org/wiki/Lie_group "Lie group"): a topological group with a compatible smooth [manifold](https://en.wikipedia.org/wiki/Manifold "Manifold") structure.
*   [Ordered groups](https://en.wikipedia.org/wiki/Ordered_group "Ordered group"), [ordered rings](https://en.wikipedia.org/wiki/Ordered_ring "Ordered ring") and [ordered fields](https://en.wikipedia.org/wiki/Ordered_field "Ordered field"): each type of structure with a compatible [partial order](https://en.wikipedia.org/wiki/Partial_order "Partial order").
*   [Archimedean group](https://en.wikipedia.org/wiki/Archimedean_group "Archimedean group"): a linearly ordered group for which the [Archimedean property](https://en.wikipedia.org/wiki/Archimedean_property "Archimedean property") holds.
*   [Topological vector space](https://en.wikipedia.org/wiki/Topological_vector_space "Topological vector space"): a vector space whose _M_ has a compatible topology.
*   [Normed vector space](https://en.wikipedia.org/wiki/Normed_vector_space "Normed vector space"): a vector space with a compatible [norm](https://en.wikipedia.org/wiki/Norm_\(mathematics\) "Norm (mathematics)"). If such a space is [complete](https://en.wikipedia.org/wiki/Complete_metric_space "Complete metric space") (as a metric space) then it is called a [Banach space](https://en.wikipedia.org/wiki/Banach_space "Banach space").
*   [Hilbert space](https://en.wikipedia.org/wiki/Hilbert_space "Hilbert space"): an inner product space over the real or complex numbers whose inner product gives rise to a Banach space structure.
*   [Vertex operator algebra](https://en.wikipedia.org/wiki/Vertex_operator_algebra "Vertex operator algebra")
*   [Von Neumann algebra](https://en.wikipedia.org/wiki/Von_Neumann_algebra "Von Neumann algebra"): a \*-algebra of operators on a Hilbert space equipped with the [weak operator topology](https://en.wikipedia.org/wiki/Weak_operator_topology "Weak operator topology").

## Universal algebra

Algebraic structures are defined through different configurations of [axioms](https://en.wikipedia.org/wiki/Axiom "Axiom"). [Universal algebra](https://en.wikipedia.org/wiki/Universal_algebra "Universal algebra") abstractly studies such objects. One major dichotomy is between structures that are axiomatized entirely by _identities_ and structures that are not. If all axioms defining a class of algebras are identities, then this class is a [variety](https://en.wikipedia.org/wiki/Variety_\(universal_algebra\) "Variety (universal algebra)") (not to be confused with [algebraic varieties](https://en.wikipedia.org/wiki/Algebraic_varieties "Algebraic varieties") of [algebraic geometry](https://en.wikipedia.org/wiki/Algebraic_geometry "Algebraic geometry")).

Identities are equations formulated using only the operations the structure allows, and variables that are tacitly [universally quantified](https://en.wikipedia.org/wiki/Universal_quantifier "Universal quantifier") over the relevant [universe](https://en.wikipedia.org/wiki/Universe_\(mathematics\) "Universe (mathematics)"). Identities contain no [connectives](https://en.wikipedia.org/wiki/Logical_connective "Logical connective"), [existentially quantified variables](https://en.wikipedia.org/wiki/Quantification_\(science\) "Quantification (science)"), or [relations](https://en.wikipedia.org/wiki/Finitary_relation "Finitary relation") of any kind other than the allowed operations. The study of varieties is an important part of [universal algebra](https://en.wikipedia.org/wiki/Universal_algebra "Universal algebra"). An algebraic structure in a variety may be understood as the [quotient algebra](https://en.wikipedia.org/wiki/Quotient_\(universal_algebra\) "Quotient (universal algebra)") of term algebra (also called "absolutely [free algebra](https://en.wikipedia.org/wiki/Free_object "Free object")") divided by the equivalence relations generated by a set of identities. So, a collection of functions with given [signatures](https://en.wikipedia.org/wiki/Signature_\(logic\) "Signature (logic)") generate a free algebra, the [term algebra](https://en.wikipedia.org/wiki/Term_algebra "Term algebra") _T_. Given a set of equational identities (the axioms), one may consider their symmetric, transitive closure _E_. The quotient algebra _T_/_E_ is then the algebraic structure or variety. Thus, for example, groups have a signature containing two operators: the multiplication operator _m_, taking two arguments, and the inverse operator _i_, taking one argument, and the identity element _e_, a constant, which may be considered an operator that takes zero arguments. Given a (countable) set of variables _x_, _y_, _z_, etc. the term algebra is the collection of all possible [terms](https://en.wikipedia.org/wiki/Term_\(logic\) "Term (logic)") involving _m_, _i_, _e_ and the variables; so for example, _m_(_i_(_x_), _m_(_x_, _m_(_y_,_e_))) would be an element of the term algebra. One of the axioms defining a group is the identity _m_(_x_, _i_(_x_)) = _e_; another is _m_(_x_,_e_) = _x_. The axioms can be represented as [trees](http://ncatlab.org/nlab/show/variety+of+algebras#examples_4). These equations induce [equivalence classes](https://en.wikipedia.org/wiki/Equivalence_class "Equivalence class") on the free algebra; the quotient algebra then has the algebraic structure of a group.

Some structures do not form varieties, because either:

1.  It is necessary that 0 ≠ 1, 0 being the additive [identity element](https://en.wikipedia.org/wiki/Identity_element "Identity element") and 1 being a multiplicative identity element, but this is a nonidentity;
2.  Structures such as fields have some axioms that hold only for nonzero members of _S_. For an algebraic structure to be a variety, its operations must be defined for _all_ members of _S_; there can be no partial operations.

Structures whose axioms unavoidably include nonidentities are among the most important ones in mathematics, e.g., [fields](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)") and [division rings](https://en.wikipedia.org/wiki/Division_ring "Division ring"). Structures with nonidentities present challenges that varieties do not. For example, the [direct product](https://en.wikipedia.org/wiki/Direct_product "Direct product") of two [fields](https://en.wikipedia.org/wiki/Field_\(mathematics\) "Field (mathematics)") is not a field, because $(1,0)\cdot(0,1)=(0,0)$, but fields do not have [zero divisors](https://en.wikipedia.org/wiki/Zero_divisor "Zero divisor").

## Category theory

[Category theory](/source/category-theory/ "Category theory") is another tool for studying algebraic structures (see, for example, Mac Lane 1998). A category is a collection of _objects_ with associated _morphisms._ Every algebraic structure has its own notion of [homomorphism](https://en.wikipedia.org/wiki/Homomorphism "Homomorphism"), namely any [function](https://en.wikipedia.org/wiki/Function_\(mathematics\) "Function (mathematics)") compatible with the operation(s) defining the structure. In this way, every algebraic structure gives rise to a [category](https://en.wikipedia.org/wiki/Category_\(mathematics\) "Category (mathematics)"). For example, the [category of groups](https://en.wikipedia.org/wiki/Category_of_groups "Category of groups") has all [groups](/source/group-mathematics/ "Group (mathematics)") as objects and all [group homomorphisms](https://en.wikipedia.org/wiki/Group_homomorphism "Group homomorphism") as morphisms. This [concrete category](https://en.wikipedia.org/wiki/Concrete_category "Concrete category") may be seen as a [category of sets](https://en.wikipedia.org/wiki/Category_of_sets "Category of sets") with added category-theoretic structure. Likewise, the category of [topological groups](https://en.wikipedia.org/wiki/Topological_group "Topological group") (whose morphisms are the continuous group homomorphisms) is a [category of topological spaces](https://en.wikipedia.org/wiki/Category_of_topological_spaces "Category of topological spaces") with extra structure. A [forgetful functor](https://en.wikipedia.org/wiki/Forgetful_functor "Forgetful functor") between categories of algebraic structures "forgets" a part of a structure.

There are various concepts in category theory that try to capture the algebraic character of a context, for instance

*   [algebraic category](https://en.wikipedia.org/wiki/Algebraic_category "Algebraic category")
*   essentially algebraic category
*   presentable category
*   [locally presentable category](https://en.wikipedia.org/wiki/Locally_presentable_category "Locally presentable category")
*   [monadic](https://en.wikipedia.org/wiki/Monad_\(category_theory\) "Monad (category theory)") functors and categories
*   [universal property](https://en.wikipedia.org/wiki/Universal_property "Universal property").

## Different meanings of "structure"

In a slight [abuse of notation](https://en.wikipedia.org/wiki/Abuse_of_notation "Abuse of notation"), the word "structure" can also refer to just the operations on a structure, instead of the underlying set itself. For example, the sentence, "We have defined a ring _structure_ on the set $A$", means that we have defined [ring](https://en.wikipedia.org/wiki/Ring_\(mathematics\) "Ring (mathematics)") _operations_ on the set $A$. For another example, the group $(\mathbb Z, +)$ can be seen as a set $\mathbb Z$ that is equipped with an _algebraic structure,_ namely the _operation_ $+$.
