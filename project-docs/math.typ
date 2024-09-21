
= Problem Statement
_In the following problem we are trying to visualize the corresponding lattice in this application for each n:_

Consider the allowed lattice symmetries for n-fold rotational axes, i.e. with a rotation
angle ϕ = 2π/n. What are the possible values for n? To do this, consider four lattice
points A, B, C, and D in the plane orthogonal to an n-fold rotation axis. Let A and B be
nearest neighbor points in the Bravais lattice with distance a, through each of which one
of the n-fold rotation axes shall pass. Let rotation about B bring A to C and rotation
about A bring B to D. Show that the geometric properties of the lattice allow only n =
1, 2, 3, 4, and 6.
Hint: Make a sketch. What conditions must the sides of the resulting trapezoid
satisfy if A, B, C, and D are lattice points?

= Basic Math stuff
== Points
- $A$: at the origin $(0,0)$
- $B$: at $(a,0)$
== Rotation Angles:
- $phi = (2 pi)/n$
== Rotations:
- Rotate $A$ around $B$ to get $C$
- Rotate $B$ around $A$ to get $D$

Rotation Matrix for rotating $P$ around $O$ by $theta$:
$ R = mat(cos theta, -sin theta; sin theta, cos theta) $
The new point is $P' = O + R times (P - O)$.
