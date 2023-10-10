# Nova
[Nova](https://eprint.iacr.org/2021/370.pdf) implementation

## Relaxed R1CS

[A Folding Scheme for NP](https://eprint.iacr.org/2021/370.pdf#page=12&zoom=100,100,780)

### [R1CS](https://eprint.iacr.org/2021/370.pdf#page=13&zoom=100,100,250)

The R1CS structure consits of sparse matrices $A, B, C \in \mathbb F^l$.

$$
(A · Z) ◦ (B · Z) = C · Z
$$

- $l$: instance length
- $m - l - 1$: witness length
- $x$: instance
- $W$: witness
- $Z$: $(W, x, 1)$

### [Relaxed R1CS](https://eprint.iacr.org/2021/370.pdf#page=14&zoom=100,100,250)

$$
(A · Z) ◦ (B · Z) = u · (C · Z) + E
$$

- $E$: error vector
- $u$: scalar
- $x$: public inputs and outputs
- $Z$: $(W, x, u)$

### [R1CS to Relaxed R1CS](https://eprint.iacr.org/2021/370.pdf#page=14&zoom=100,100,370)

The R1CS instance can be expressed as a relaxed R1CS instance by $u = 1$ and $E = 0$.

$$
(A · Z) ◦ (B · Z) = 1 · (C · Z) + 0
$$

### [Commitment](https://eprint.iacr.org/2021/370.pdf#page=14&zoom=100,100,850)

- $pp_W$: commitment vectors for $W$ size $m$
- $pp_E$: commitment vectors for $E$ size $m - l - 1$
- $(\overline E, u, \overline W, x)$: committed relaxed R1CS instance
- $u$: scalar
- $x$: public inputs and outputs
- $\overline E$: $Com(pp_E, E, r_E)$
- $\overline W$: $Com(pp_W, W, r_W)$

### [Committed Relaxed R1CS](https://eprint.iacr.org/2021/370.pdf#page=15&zoom=100,100,210)

$$
(\overline E, u, \overline W, x)
$$

- $E$: error vector (0 when the R1CS is initalized)
- $u$: scalar (1 when the R1CS is initalized)
- $W$: witness
- $x$: instance

$(\overline E, u, \overline W, x)$ is satisfied by a witness $(E, r_E, W, r_W)$

if
- $\overline E = Com(pp_E, E, r_E)$
- $\overline W = Com(pp_W, W, r_W)$
- $(A · Z) ◦ (B · Z) = u · (C · Z) + E$

## [Folding Scheme](https://eprint.iacr.org/2021/370.pdf#page=10&zoom=100,100,650)

$R$ is a relation over public parameters, structure, instance, and witness tuples.

A folding scheme for $R$ consists of

- $g$: probabilistic polynomial time generator algorithm
- $K(pp, s) \rightarrow (pk, vk)$: On input $pp$ and common structure $s$ between instances to be folded, outputs a prover key $pk$ and a verifier key $vk$
- $P(pk, (u_1,w_1),(u_2,w_2)) \rightarrow (u,w)$: On input instance-witness tuples $(u_1,w_1)$ and $(u_2,w_2)$ outputs a new instance-witness tuple $(u,w)$ of the same size
- $V(vk,u_1,u_2) \rightarrow u$: On input instances $u_1$ and $u_2$, outputs a new instance $u$

## IVC (Incrementally Verifiable Computation)
[An IVC Scheme with Proof Compression](https://eprint.iacr.org/2021/370.pdf#page=16&zoom=100,100,790)

### [NIFS](https://eprint.iacr.org/2021/370.pdf#page=16&zoom=100,100,490)

$$
(g, K, P, V)
$$

- $G$: output $pp \leftarrow G(1^λ)$
- $K(pp,(A,B,C))$: $vk \leftarrow p(pp,s)$ and $pk \leftarrow (pp, (A,B,C), vk)$; output(vk, pk)
- $P(pk,(u_1,w_1), (u_2,w_2))$: $r \leftarrow p(vk,u_1,u_2,\overline T)$ output result
- $V(vk,u_1,u_2,\overline T)$: $r \leftarrow p(vk,u_1,u_2,\overline T)$ output result
- $p$: cryptographic hash function

## Pedersen Commitment

For vectors over $\mathbb F^m$

- $pp \rightarrow Gen(1^λ,m)$: Sample $g \rightarrow R \mathbb G^m, h \rightarrow R \mathbb G$. Output $(g, h)$
- $C \rightarrow Com(pp, x \in \mathbb F^m, r \in \mathbb F$: Output $h^r·\prod_{i\in [0,...,m]}g_i^{x_i}$
