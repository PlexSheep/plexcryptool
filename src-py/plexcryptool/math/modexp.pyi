"""
modular exponentiaton

Implements fast exponentiation with applied modulo. Usefull for calculations in a gallois
field.

___
@Author:     Christoph J. Scherr <software@cscherr.de>
@License:    MIT
@Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
"""

def modular_exponentiation(
        base: int, 
        orig_exp: int, 
        field: int, 
        verbose: bool
        ) -> int:
    """
    perform modular exponentiation

    :param base the base of the exponentiation
    :param orig_exp the exponent of the base
    :param field the number that describes the gallois field (should be prime)
    :param verbose print steps
    """
    ...
