# Last Ditch Project

## Synopsis

After massive urban growth on a remote planet, the existing governments collapsed into chaos. Most people escaped the planet to more stable locations. The largely abandoned planet is now home to people trying to survive in the ruins of the towering skyscrapers. The most capable have taken over the highest floors, leaving the others below to fend for themselves. 

The user starts in the lowest occupied area of a skyscraper. These levels are overrun with waste and junk that is mined to produce the basic necessities of life. The floors above these are home to the weakest and most disorganized factions where the user can first establish themselves. The higher floors have stronger and more well-established factions. They attempt to control access to these floors. 

## Features

* The main character has the skills to become a community leader
* The leader will have to deal with disloyalty based on quality of faction life
* They can pass this position on to a new generation
* They help the community survive and grow
* The community can restructure the building to better fit their needs
* The building is visited by traders/scavengers from other areas 
* Two main languages are spoken in the building: English and Essiah 

## Implementation Notes

* Real-time turn system
* VERY consequential damage and social instability from fighting
* Characters can move through each other
* Camera controlled by WASD, character movement point/click
* Wiring and plumbing in walls and floors, possibility of developing wireless
* Lighting setup using transparent black overlays

## Characters

The main character can join or create a faction.

Characters can partner together into relationships of different kinds with characters inside and outside of their faction. Characters can also have children, although children between members of rival factions will cause tensions, and they will not immediately belong to either faction. 

The children of a character can take over as leader, and this will pass the main character role on to the child. Faction leadership can also be passed on to a trusted member of the faction who is not related to main character. In both cases, the faction's trust of the new leader will have consequences.

Every member of the faction has a series of traits that define how they interact with the faction and the world around them:

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

As the player's faction gains more status and relocates to higher floors, they become a larger target of scavangers and rival factions, but they also attract the attention of more wealthy traders and powerful recruits.

## Map

The map is comprised of all the upper floors of a massive skyscraper that is randomly generated at the beginning of each simulation.

It consists of four major sections:
* Sublevels - Abandoned floors below the lowest inhabited floor
* Lowlevels - The initial floors sparsely populated and the least organized
* Midlevels - Floors containing many average strength factions
* Upperlevels - Floors containing the strongest and most well-equipped factions

Each floor is a large tiled map viewed from above with a number of layers.
1. Floor - The floor
2. Wall - The building structures (i.e. walls, doors, windows)
3. Object - Furniture and other objects like machinery 
4. Entity - Entities like humans or animals
5. Overlay - User interface tiles like selection

### Environment

The map experiences a day/night cycle as well as seasons. 

Throughout the year, the building is visited by various traders and scavengers looking to take advantage of the outpost.

## Production

By building different tools and developing new production techniques, the faction is able to combine its skills to produce higher quality items to aid in its survival.

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

