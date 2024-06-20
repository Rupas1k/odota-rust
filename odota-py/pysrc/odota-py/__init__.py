from enum import Enum

from odota_py import odota_py
from . import odota_py

class EntryType(Enum):
    ANY = "any"
    COSMETICS = "cosmetics"
    DOTAPLUS = "dotaplus"
    EPILOGUE = "epilogue"
    CHAT_WHEEL = "chatwheel"
    CHAT = "chat"
    OBS = "obs"
    OBS_LEFT = "obs_left"
    SEN = "sen"
    SEN_LEFT = "sen_left"
    PINGS = "pings"
    ACTIONS = "actions"
    DRAFT_START = "draft_start"
    DRAFT_TIMINGS = "draft_timings"
    PLAYER_SLOT = "player_slot"
    INTERVAL = "interval"
    STARTING_ITEMS = "StartingItems"
    ABILITY_LEVEL = "DotaAbilityLevel"
    # DotaCombatlogTypes
    INVALID = "DotaCombatlogInvalid"
    DAMAGE = "DotaCombatlogDamage"
    HEAL = "DotaCombatlogHeal"
    MODIFIER_ADD = "DotaCombatlogModifierAdd"
    MODIFIER_REMOVE = "DotaCombatlogModifierRemove"
    DEATH = "DotaCombatlogDeath"
    ABILITY = "DotaCombatlogAbility"
    ITEM = "DotaCombatlogItem"
    LOCATION = "DotaCombatlogLocation"
    GOLD = "DotaCombatlogGold"
    GAME_STATE = "DotaCombatlogGameState"
    XP = "DotaCombatlogXp"
    PURCHASE = "DotaCombatlogPurchase"
    BUYBACK = "DotaCombatlogBuyback"
    ABILITY_TRIGGER = "DotaCombatlogAbilityTrigger"
    PLAYERSTATS = "DotaCombatlogPlayerstats"
    MULTIKILL = "DotaCombatlogMultikill"
    KILLSTREAK = "DotaCombatlogKillstreak"
    TEAM_BUILDING_KILL = "DotaCombatlogTeamBuildingKill"
    FIRST_BLOOD = "DotaCombatlogFirstBlood"
    MODIFIER_STACK_EVENT = "DotaCombatlogModifierStackEvent"
    NEUTRAL_CAMP_STACK = "DotaCombatlogNeutralCampStack"
    PICKUP_RUNE = "DotaCombatlogPickupRune"
    REVEALED_INVISIBLE = "DotaCombatlogRevealedInvisible"
    HERO_SAVED = "DotaCombatlogHeroSaved"
    MANA_RESTORED = "DotaCombatlogManaRestored"
    HERO_LEVELUP = "DotaCombatlogHeroLevelup"
    BOTTLE_HEAL_ALLY = "DotaCombatlogBottleHealAlly"
    ENDGAME_STATS = "DotaCombatlogEndgameStats"
    INTERRUPT_CHANNEL = "DotaCombatlogInterruptChannel"
    ALLIED_GOLD = "DotaCombatlogAlliedGold"
    AEGIS_TAKEN = "DotaCombatlogAegisTaken"
    MANA_DAMAGE = "DotaCombatlogManaDamage"
    PHYSICAL_DAMAGE_PREVENTED = "DotaCombatlogPhysicalDamagePrevented"
    UNIT_SUMMONED = "DotaCombatlogUnitSummoned"
    ATTACK_EVADE = "DotaCombatlogAttackEvade"
    TREE_CUT = "DotaCombatlogTreeCut"
    SUCCESSFUL_SCAN = "DotaCombatlogSuccessfulScan"
    END_KILLSTREAK = "DotaCombatlogEndKillstreak"
    BLOODSTONE_CHARGE = "DotaCombatlogBloodstoneCharge"
    CRITICAL_DAMAGE = "DotaCombatlogCriticalDamage"
    SPELL_ABSORB = "DotaCombatlogSpellAbsorb"
    UNIT_TELEPORTED = "DotaCombatlogUnitTeleported"
    KILL_EATER_EVENT = "DotaCombatlogKillEaterEvent"
    NEUTRAL_ITEM_EARNED = "DotaCombatlogNeutralItemEarned"


class OdotaPy:
    def __init__(self):
        self.callbacks = {}

    def on(self, type: EntryType):
        type = type.value

        def inner(func):
            if type not in self.callbacks:
                self.callbacks[type] = []
            self.callbacks[type].append(func)
            return func

        return inner

    def parse(self, binary: bytes):
        output = odota_py.parse_replay(binary)
        if len(self.callbacks) == 0:
            return

        for entry in output:
            if entry.type in self.callbacks:
                for callback in self.callbacks[entry.type]:
                    callback(entry)
            if "any" in self.callbacks:
                for callback in self.callbacks["any"]:
                    callback(entry)
