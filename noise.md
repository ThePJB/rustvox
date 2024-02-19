1. try adding in gradients to determine vegetation
2. operation: lerp sharply.
    lerp sharply but one is strictly bigger, cliffy

3. warp same vs different seed
4. warp different function
5. island chains: domain repitition for one mask layer etc
6. ridge noise
7. warping masks


# Domain Warping
domain warping of a big low frequency function with a higher frequency one produces some nice results

managed to make some nice looking mountains. it would be good to make them snow capped or otherwise change up some of the appearance
more aggressive or spikey in areas maybe
height remap maybe
erosive vs spikey

cliffs, how do they work


could use some kind of FIR to have a more eroded side


normalCliffy is probably less realistic but more interesting from a gameplay perspective

maybe this is ok as a sort of minimal, blank canvas


might be a juicy abstraction for world gen, might even be able to have knobs you can edit
like generators combine other generators...
maybe just put them in different functions


thinking mountains could also do with sharp rocks
sharp actually looks ok, needs a bit of a rework to apply to each individual noise layer
its going to look quite good with the gradient thing

amit touches on the exponential thing with noise

can absolutely have stupid stuffing big mountains as a fantasy biome with something on the top, guarded by flying enemies or something, maybe surrounded with cloud lol

imagine having a domain repitition esque structure where they just got bigger and bigger, acid world
big fibo spiral or something


beach transects
bays barriers headland - remapping the ocean function in a non monotonically increasing way
lagoon atoll?
wetland/swamp/marsh/mire gen
mountain gen
glue together


modulus noise for cheeky cusps, maybe for magic walls or something


could try modeling different stone hardness and therefore erosion

fluvial terraces
a fantasy terrace biome would be cool anyway

valley transect
terraces
terraces + sediment

rock strata would be nice

canyons / gorges: cliffs on the sides
more common in arid regions because weathering is more localized. carved into a tableland rather than kinda between mountains as a valley is

anyway next i could do:
    mountains 2: sharper and with erody deposits
    desert + canyons (but no rivers lel)
    beaches
    wetlands

magic stuff can have all kinds of structure like domain repetition, occurring in concentric circles etc. like for hell, circles of hell.

maybe i should amplify the gradient somehow...
https://www.decarpentier.nl/scape-procedural-extensions
its actually used in fbm to tweak the noise of finer octaves, its pretty fair you want more hf in the cliffy side bits i think

cool well im spent
maybe organize the world gen a bit better. stuff around more with mountains or do something different