# Last Ditch Project

## Synopsis

After massive urban growth on a remote planet, the existing governments collapsed into chaos. Most people escaped the planet to more stable locations. The largely abandoned planet is now home to people trying to survive in the ruins of the towering skyscrapers. The most capable have taken over the highest floors, leaving the others below to fend for themselves. It's become a place of refuge for people looking to escape the eye of more civilized planets. 

The starting level is the lowest occupied area of the skyscraper with the least established factions. The levels below this are almost entirely overrun with waste and junk that is mined to produce the basic necessities of life. As the floors rise above this, stronger and more well-established factions have staked out territories. They attempt to prevent access from the scavengers below.

## Features

* Play as a community leader
* Continue to pass on leadership to new generations
* Help the community survive and grow
* Rebuild the structure of the skyscraper
* Interact with visiting traders/raiders from outside the skyscraper
* Implement bi-lingual English/Essiah

## Characters

Each character can belong to a faction, but some will operate as individuals for periods of time.

Characters can partner together into relationships of different kinds with characters inside and outside of their faction. Characters can also have children, although children between members of rival factions will cause tensions, and they will not immediately belong to either faction. 

Every character has a series of characteristics that define how they interact with the faction and the world around them:

### Traits
Main Personality:
* Openness
* Conscientiousness
* Extraversion
* Agreeableness
* Neuroticism

Each member also has a series of tastes that affect how certain actions change their mood.

### Needs
* Hunger
* Thirst
* Fatigue
* Social
* Wellbeing

### Attributes
* Strength
* Constitution
* Agility
* Dexterity
* Intelligence
* Willpower
* Charisma

### Skills
* Botany
* Brewing
* Chemistry
* Gunsmith
* Construction
* Management
* Metalwork
* Manufacturing
* Electronics
* Software
* Trade

### Equipment

Each character can equip items in these areas:
* Head
* Eyes
* Undershirt
* Shirt
* Jacket
* Back
* Waist
* Left Arm
* Right Arm
* Left Hand
* Right Hand
* Left Leg
* Right Leg
* Left Foot
* Right Foot

### Health Problems

Characters can undergo a series of health problems that will affect their behavior and performance. Some of these are caused by the character's actions throughout the game and others are random.

* Physical Problems
  1. Heart Disease
  2. Diabetes
  3. Cancer
* Psychological Problems
  1. Mental Retardation
  2. Sociopathy
  3. Psychopathy

## Factions

The player forms a faction at the beginning of the game with two other outcasts who have just arrived at the lowest level. As their faction grows in influence, they will be able to recruit characters from other, stronger factions, and there will also be periodic arrivals to the building looking for opportunity. More capable characters arrive at the higher floors looking for a faction to join. 

As the player's faction gains more status and relocates to higher floors, they become a larger target of scavangers and rival factions.

## Map

The map is comprised of all the upper floors of a massive skyscraper that is randomly generated at the beginning of each simulation.

It consists of four major sections:
* Sublevels - Abandoned floors below the lowest inhabited floor
* Low - The initial floors sparsely populated and the least organized
* Mid - Floors containing many average strength factions
* High - Floors containing the strongest and most well-equipped factions

Each floor is a large tiled map viewed from above with a number of layers.
1. The floor itself
2. Anything placed directly on the floor.
3. Furniture and other structural objects like walls 
4. Anything placed directly on the objects from layer 3

The individual characters are placed on top of the map and move between tiles. They can move in all 8 directions from their current tile to any adjacent tile.

### Environment

The map experiences a day/night cycle as well as passing years. 

Throughout the year, the building is visited by various traders and scavengers looking to take advantage of the outpost.

## Production

By building different new tools and developing new production techniques, the faction is able to combine its skills to produce higher quality items to aid in its survival.

### Production Facilities
* Hydroponics
* Metal Fabricator
* Plastic Manufacturer
* 3D Printer
* Distillery
* Filtration System
* Milling Machine
* Computer Station
* Textile Production

### Materials
* Steel
* Plastic
* Silicon
* Gold
* Silver
* Zinc
* Oil
* Fuel

### Items
* Drinks
  * Water, dirty
  * Water, clean
  * Vegetable juice
  * Fruit juice
* Food
  * Ration
  * Meat
  * Fruits
  * Vegetables
* Drugs
  * Beer
  * Wine
  * Spirits
  * Cannabinoid
  * Amphetamine
* Weapons
  * Handgun
  * Pulsegun
  * Shock Baton
  * Drone
  * Fragmentation Mine
* Shields
  * Riot
  * Magnetic

## Implementation Details
* Real-time turn system
* VERY consequential damage to characters/fighting costly for both sides
* Characters can move through each other
