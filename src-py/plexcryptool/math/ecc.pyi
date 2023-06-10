"""
Elliptic curve cryptography

___
@Author:     Christoph J. Scherr <software@cscherr.de>
@License:    MIT
@Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
"""
from src-py.plexcryptool.math.gallois import GalloisFiled


class EllipticCurve:
    """
    an elliptic curve
    """
    def __init__(self, field: GalloisFiled, a: int, b: int, verbose = True) -> None: 
        """
        constructor
        """
        ...

    def new_point(self, r: int, s: int) -> EllipticCurvePoint: 
        """
        generate a new point in the curve
        """
        ...

    def poly(self, x: int, y: int) -> int:
        """
        insert into the curves polynomial
        """
        ...

    def check_point(self, p: EllipticCurvePoint) -> bool:
        """
        check if the point is valid
        """
        ...

    def add(self, p1: EllipticCurvePoint, p2: EllipticCurvePoint) -> EllipticCurvePoint:
        """
        add two points
        """
        ...

    def mul(self, p: EllipticCurvePoint, t: int) -> EllipticCurvePoint:
        """
        multiply a point by an integer
        """
        ...

    def get_infinity_point(self) -> EllipticCurvePoint:
        """
        get the infinity point of a curve
        """
        ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...


class EllipticCurvePoint:
    """
    represent a point on some curve
    """
