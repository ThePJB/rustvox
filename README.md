# Problems
* frame time information
* decouple chunk loading across time
* multithread chunk generation
* Need debug information, stupid holes in world
    It seems like its literally doing everything correctly and then drawing the wrong fucking thing on the screen
* Remove glam
* unloading might be slightly lazy
* weird shit when you go in the ground
* add culling
* frustum cull chunks
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