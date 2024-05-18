# Copyright (c) 2024, Lukas Heindl
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# 3. Neither the name of the copyright holder nor the names of its
#    contributors may be used to endorse or promote products derived from
#    this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
# SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
# CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
# OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

from decimal import Decimal, ROUND_HALF_UP

"""
One implementation of how seats can be distributed
"""
class Sainte_Lague_Schepers_utils:
    def __init__(self, zweitwahl, ges_sitze):
        self.zweitwahl = zweitwahl
        self.ges_sitze = ges_sitze
        self.verteilung = {}

    def __str__(self):
        return "{[sls_utils] zweitwahl:" + str(self.zweitwahl) + ", ges_sitze:" + str(self.ges_sitze) + ", verteilung:" + str(self.verteilung) + "}"
    def __repr__(self):
        return str(self)

    @property
    def zweitwahl(self):
        return self._zweitwahl
    @zweitwahl.setter
    def zweitwahl(self, zweitwahl):
        if isinstance(zweitwahl, dict):
            self._zweitwahl = zweitwahl
        else:
            raise AttributeError("zweitwahl has to be a dict, but was", type(zweitwahl))
    @property
    def ges_sitze(self):
        return self._ges_sitze
    @ges_sitze.setter
    def ges_sitze(self, ges_sitze):
        if isinstance(ges_sitze, int):
            self._ges_sitze = ges_sitze
        else:
            raise AttributeError("ges_sitze has to be an int, but was", type(ges_sitze))
    @property
    def verteilung(self):
        return self._verteilung
    @verteilung.setter
    def verteilung(self, verteilung):
        if isinstance(verteilung, dict):
            self._verteilung = verteilung
        else:
            raise AttributeError("verteilung has to be a dict, but was", type(verteilung))

    """
    calculate distribution
    """
    def apply(self):
        zutDivisor = sum([x for _,x in self.zweitwahl.items()]) / Decimal(self.ges_sitze)
        while (s := sum([x for _,x in self.verteilung.items()])) != self.ges_sitze: #TODO passen die steps so, oder kann man die irgendwie schlau berechnen (oder immer +-10% oder so) check ob osziliert -> steps kleiner machen (dann aber bei 100 anfangen)
            if s < self.ges_sitze:
                zutDivisor -= 1
            elif s > self.ges_sitze:
                zutDivisor += 1
            self.updateVert(zutDivisor)
        # print("Divisor:", zutDivisor, self.verteilung)
        # print("sitze:", self.ges_sitze, "zutDivisor:", zutDivisor)
        return self.verteilung

    def updateVert(self, zutDivisor):
        for p, stimmen in self.zweitwahl.items():
            sitze = (stimmen / zutDivisor).to_integral_value(rounding=ROUND_HALF_UP) #TODO .5 -> würfeln nicht beachtet
            self.verteilung.update({p: int(sitze)})
