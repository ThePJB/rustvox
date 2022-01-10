Tablelands, escarpments, Buttes, Hoodoos.

I've kinda got buttes from the remapping.

With noise theres always half and anti.
Dont want anti buttes though lol. Maybe you could use a neighbourhood sample. That might have some interesting effect, noise + neighbourhood sample.
Might be able to achieve tablelands with that


Tablelands need a certain shape. What if you have one thresholded noise map and another that checks and fills until its naked on 3 sides or something
want big regions with interesting edges. LF but domain warped maybe.


badlands too.

Vegetation and soil and stuff based on gradient.

river canyons.





Rivers would be great. Need a low res map. Could do voronoi for computing them and splines for integrating them. Splines could include river canyons.
what to do about the local minimas though? Maybe could plot local minimas and catchments, watersheds. Continents need to be shaped in a compatible way with reality.
water table and aquifers?
18% endorheic. Maybe could just make some big inland lakes. flood fill and if they go above some threshold they get outlets
also waterfalls.
Permeability / solubility of stone, matters for soil and erosion and stuff too.

glaciers and fjords would be awesome too, maybe in the same way as rivers. Whats with glaciers that are halfway down a fjord or whatever





Role of simulation? Seems like more emergence but more expensive and hectic. Could be fun to muck around with that idea from before with the CA.
Plate tectonics simulation, probably difficult but I wonder if any worthwhile simplifications of it exist







archaeological features. Especially really subtle ones with maybe some kind of magic significance


non monotonic remapping could totally be done, maybe for magic leylines or something

convolution technique is quite interesting
right now its very discrete and also linear

bigger simpler kernels could do something
if you want repitition tyhis achieves it especially for magic like hexagon shaped spawns etc

you could totally do filtering like this FIR as well or other techniques lol. gaussian blur.

-------

Beach:

Warp w/ beachness s/t lines of equal beachness vary slowly but perpendicular not at all?
or just vary very low freq spatially might not be a problem

got it started its not very convincing yet but we'll get there!
transition from dune back to land is shit
i think its meant to get higher and then come back down, needs another transition function
or up, in case of headland, could be a discontinuous lerp thing


good erosion: have one (discontinuous) map for hardness of rock and one continuous one for level of erosion, should == cliffs

structures: a special kind of chunk unioned with the ones it needs to touch

doing it with cos is a cunt
how about bezier curves for transects?


do fog that will be nice

do procedural grass that will be nice, can do height as well, outside of block system
maybe wind in vertex shader or something, mm triangles


probably better to truncate the dunes than compress them but I digress

for a better structure we could totally do a distance function thing and then also potentially classify for dank estuaries and stuff


need cubic beziers to have enough good shape I think
need to sort out the interface between the ocean and the beach, and the beach and the land

and I think the distance function thing can make it have nice properties as far as smoothness and stuff is concerned




