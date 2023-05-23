"""
gallois field calculations

Implements a number of calculations in gallois fields. Use the GalloisFiled struct/class and it's methods.


___
@Author:     Christoph J. Scherr <software@cscherr.de>
@License:    MIT
@Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
"""

class GalloisFiled:
    def __init__(self, base: int, verbose: bool) -> None:
        """
        Create a new Gallois field
        """
        ...

    def reduce(self, n: int) -> int:
        """
        reduce the given number to fit into the field
        """
