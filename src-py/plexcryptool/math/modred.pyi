"""
modular reduction

Implements automatic modular reduction in a field specified by a given relation.

Basically, any binary number can be written as a polynomial. This polynomial can be reduced by
the relation that defines a field. In that field. This is what we call modular reduction.

___
@Author:     Christoph J. Scherr <software@cscherr.de>
@License:    MIT
@Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
"""

def modred(poly: int, relation: int, verbose: bool) -> int:
    """
    perform modular reduction

    :param poly the polynomial as int
    :param relation the relation as int
    :param verbose print steps
    """
    ...
