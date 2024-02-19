TODO
-------

add worldgen features, play with generators whatever

Direction culling?

Water z buffer shader would be pretty easy, learn something new, kinda a ceebs lol

distance fog: fragment.z


Worldgen remarks
----------------
* be fun to just try and recreate various environments
 * yellowstone - how they get sulfurous pools, actually I kinda get it, not sure how you would get the height etc
 * craggy spikey peaks
 * an island - just to have a sensible large scale structure

* like remapping heights as a concept
* could simplify the trait to have an output to 2d and a input thats just xyz, 2d thing, meh

* what can be made direct, probably could do trees with the right noise function if you  just do a quick local numerical search on it
* so much shit you can do on noise function really
* newtons method on perlin noise -> blue noise?
* maybe you could have a direct formula for still water if you detected a local minimum.. but nah
* but i wonder what effects you could get with neighbourhood sampling, e.g. valley microclimate

* headlands are cool
* headland: a component of noise smooth minned into normal landscape, clamped hard by coastline mask
 * maybe a general approach to cliffs as well. like the sand and stuff at the footing though...

* can probably do some cool shit with edge detection for sulfite pools maybe
 * reminds me of the roots water in aoe2

* dark world with super gay swamp areas that suck to traverse
 * continent
 * swamp
 * swamp island
 * water
 make em feel real bad by towering over
could have a real simple classifier to say if a tile e.g. it satisifes X and neighbours N away also satisfy X
classifier biomes could be spicy than traditional forward minecraft biomes


periodic could be fine for real big shit tbh, it could even be chunk aligned and just a bool yes or no for each chunk

what about periodic but domain warped
play with domain warping hey. is normal just like domain warping the identity function?

other avenues: rivers - they are cool but might not be worth the effort
do want some larger scale structure though

I think for now and potentially for wizard game, just play with interesting and dangerous biomes made from mucking about with noise
ice crevasses too

one day its gonna be time for player controller and entities and ai etc

obelisks with stuff on top

theres a subcategory of behaviour you can get with direct methods
if you had 2 things with coprime periods you could maybe get some cheeky fake blue noise happening but idk
probably worth it to implement 'cheap indirect' as a general thing - quadtree or something lookup with radius

# Problems
* Need debug information, stupid holes in world
    It seems like its literally doing everything correctly and then drawing the wrong fucking thing on the screen
* Remove glam
* unloading might be slightly lazy
* weird shit when you go in the ground
* lel mouse scrolling is on a different axis and im not handling it
* screenshot button

in chunk right or what
mesh could be wrong?

how many things in an ebo lol


smaller solid voxels look real clean hey
remember to capture beauty and also to be anti grind

whats beautiful? boids flocking AI
collaborative AI
beautiful structures
vistas



generation: squish range to simulate erosion
then that in and of itself could be a parameter

man accidentally made big discontinuities for cliffs, very cool
do that

* greedy meshing: do it face at a time?
* or can you mesh first and do it from the buffer? - intermediate format for faces? then runs would be super easy
greedy meshing squares but maybe you would identify breaks or something. probs dont get too fancy lol
basically just faces + RLE. doing square shit is hectic but you could do it if it would be worth it. rle strips to squares and rle strips
rle chunks in memory??? until written or something. lighting would be like nah lad, but you could separate
how about mesh, maybe an rle chunk actually gets meshed super easy....

chunk system 

maybe chunks should just always be rle bro
oh but muh random access, i mean like right now its fine lol
maybe you could have some kinda data structure for column axis, do queries like highest X

it would achieve a fucked up level of compression like 95%


seems to be some issues with floating point accuracy
try wonkifying organic blocks like grass
randomize colour of grass

have some nice distance fog
maybe you can blend it into the lodmesh or whatever. world ending is shit
loaded chunks volume: wider probably good
is air expensive or nah?
earth curvature transofrmation

backface culling


sort front to back for rendering or whatever

multithreading generation means separating chunk data from gpu shit

something is wrong with unloading or something

+z fucked for winding order
also framerate dependent physics I think, get that shit straight with opengl hey

could have mad pokey uppey boys for cliffs. just discontinuous ay. the range remapping could do that. again it could be also varied, or sit on top of things, etc

how about overhangs? hmm
combine 2 heightmaps? could be decent for cliffs too

and make it directional and domain warped

you could imagine a cheeseworld though where its got a bunch of different thresholds or'ed together and then theres somethign in the pits
cenotes or lava holes or something

correlating the right things

omg contour detection for island chains, craters etc...

discontinuities are mad and look like erosion
could have an erosion bit around the edges of the cliffs too

man I was really frothing that one cliff beach like cliffs of dover or whatever, there must be a way to emphasize that. like you a cliff would be edge detectable... hmmm

and gameplay wise the cliffs thing is pretty cool too

dumb chunk priority w/rt height if theres a big discontinuity
could check 4 corners

blue and white magic place mega craters

real big macroscopic structures
big columns


biomes relate to each other in certain ways like the lowlands and highlands in hell
essentially add enough stuff that it doesnt feel like its just all the same
things need to matter

so yeah big craters, try use contours more

floating islands
cool cavern layers

realms you can only enter at certain times eg moon land only during certain phases
crater lake moon thing

steamy geothermal crater lake land


well, frustum cull wasnt working
maybe to it geometrically: 4 planes and object is kept if any part of it is inside said planes, cbf right now migth try greedy meshing

multithreading world gen is probably the optimization im going to feel the most.. and keeping it off main thread
requires splitting chunkdata from gpu stuff

have nice behaviour on windows loses focus

now mesh time is the killer and gpu rendering is the bottleneck
can also do sorting
still dont know if its doing huge copies or not

meshing water separately
trees
could have translucent blocks: opaque if N deep... like a certain crystal or something

could use a different shader and not have opacity be sent to gpu for solid blocks...

we could sort chunks before we generate

transparency still a bit cooked, probably very cooked if water is in the wrong spot
need to sort inside chunk too probbaly

hmm pixely displacement or bump mapping for nearby geometry looks pretty comfy, see voxelnauts


hmmmm SVO??

maybe do trees


cull by facing direction smart!


ok meshing is laggy
make it mesh way less per frame and sort by eg distance and being in view
also move meshing into MT


could dither in the stone from cliffs with hf noise
grass on top, sand on top

do nice fog

-----------
world gen: composition of functions and derivatives of functions

could you have it be low res with a clean dithering shader and limited 3d, but voxels
so basically all 1 colour or something and then 

damn imagine 2d silhouetted leggy boy billboards for enemies with IK

it sounds like it looks cool, I hope combat is satisfying, I guess it would mostly be casting spells


just generate meshes
generate elevation changes, boulders, cover, etc

could even have fps vibes
I always just gravitate to open world wizards
its the 3d version of wizrad