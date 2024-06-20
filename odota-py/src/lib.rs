use odota_core::{parse_replay, Entry};
use pyo3::prelude::*;

#[pyclass]
pub struct PyEntry {
    inner: Entry,
}

#[pymethods]
impl PyEntry {
    #[getter]
    pub fn time(&self) -> i32 {
        self.inner.time as i32
    }

    #[getter]
    pub fn r#type(&self) -> Option<String> {
        self.inner.r#type.clone()
    }

    #[getter]
    pub fn team(&self) -> Option<i32> {
        self.inner.team
    }

    #[getter]
    pub fn unit(&self) -> Option<String> {
        self.inner.unit.clone()
    }

    #[getter]
    pub fn key(&self) -> Option<String> {
        self.inner.key.clone()
    }

    #[getter]
    pub fn value(&self) -> Option<u32> {
        self.inner.value
    }

    #[getter]
    pub fn slot(&self) -> Option<i32> {
        self.inner.slot
    }

    #[getter]
    pub fn player_slot(&self) -> Option<i32> {
        self.inner.player_slot
    }

    #[getter]
    pub fn player1(&self) -> Option<i32> {
        self.inner.player1
    }

    #[getter]
    pub fn player2(&self) -> Option<i32> {
        self.inner.player2
    }

    #[getter]
    pub fn attackername(&self) -> Option<String> {
        self.inner.attackername.clone()
    }

    #[getter]
    pub fn targetname(&self) -> Option<String> {
        self.inner.targetname.clone()
    }

    #[getter]
    pub fn sourcename(&self) -> Option<String> {
        self.inner.sourcename.clone()
    }

    #[getter]
    pub fn targetsourcename(&self) -> Option<String> {
        self.inner.targetsourcename.clone()
    }

    #[getter]
    pub fn attackerhero(&self) -> Option<bool> {
        self.inner.attackerhero
    }

    #[getter]
    pub fn targethero(&self) -> Option<bool> {
        self.inner.targethero
    }

    #[getter]
    pub fn attackerillusion(&self) -> Option<bool> {
        self.inner.attackerillusion
    }

    #[getter]
    pub fn targetillusion(&self) -> Option<bool> {
        self.inner.targetillusion
    }

    #[getter]
    pub fn abilitylevel(&self) -> Option<u8> {
        self.inner.abilitylevel
    }

    #[getter]
    pub fn inflictor(&self) -> Option<String> {
        self.inner.inflictor.clone()
    }

    #[getter]
    pub fn gold_reason(&self) -> Option<u32> {
        self.inner.gold_reason
    }

    #[getter]
    pub fn xp_reason(&self) -> Option<u32> {
        self.inner.xp_reason
    }

    #[getter]
    pub fn valuename(&self) -> Option<String> {
        self.inner.valuename.clone()
    }

    #[getter]
    pub fn gold(&self) -> Option<u32> {
        self.inner.gold
    }

    #[getter]
    pub fn lh(&self) -> Option<u16> {
        self.inner.lh
    }

    #[getter]
    pub fn xp(&self) -> Option<u16> {
        self.inner.xp
    }

    #[getter]
    pub fn x(&self) -> Option<u8> {
        self.inner.x
    }

    #[getter]
    pub fn y(&self) -> Option<u8> {
        self.inner.y
    }

    #[getter]
    pub fn z(&self) -> Option<u8> {
        self.inner.z
    }

    #[getter]
    pub fn stuns(&self) -> Option<f32> {
        self.inner.stuns
    }

    #[getter]
    pub fn hero_id(&self) -> Option<i32> {
        self.inner.hero_id
    }

    #[getter]
    pub fn itemslot(&self) -> Option<u8> {
        self.inner.itemslot
    }

    #[getter]
    pub fn charges(&self) -> Option<u8> {
        self.inner.charges
    }

    #[getter]
    pub fn secondary_charges(&self) -> Option<u8> {
        self.inner.secondary_charges
    }

    #[getter]
    pub fn life_state(&self) -> Option<u8> {
        self.inner.life_state
    }

    #[getter]
    pub fn level(&self) -> Option<u8> {
        self.inner.level
    }

    #[getter]
    pub fn kills(&self) -> Option<u8> {
        self.inner.kills
    }

    #[getter]
    pub fn deaths(&self) -> Option<u8> {
        self.inner.deaths
    }

    #[getter]
    pub fn assists(&self) -> Option<u8> {
        self.inner.assists
    }

    #[getter]
    pub fn denies(&self) -> Option<u8> {
        self.inner.denies
    }

    #[getter]
    pub fn entityleft(&self) -> Option<bool> {
        self.inner.entityleft
    }

    #[getter]
    pub fn ehandle(&self) -> Option<u32> {
        self.inner.ehandle
    }

    #[getter]
    pub fn obs_placed(&self) -> Option<u8> {
        self.inner.obs_placed
    }

    #[getter]
    pub fn sen_placed(&self) -> Option<u8> {
        self.inner.sen_placed
    }

    #[getter]
    pub fn creeps_stacked(&self) -> Option<u8> {
        self.inner.creeps_stacked
    }

    #[getter]
    pub fn camps_stacked(&self) -> Option<u8> {
        self.inner.camps_stacked
    }

    #[getter]
    pub fn rune_pickups(&self) -> Option<u8> {
        self.inner.rune_pickups
    }

    #[getter]
    pub fn repicked(&self) -> Option<bool> {
        self.inner.repicked
    }

    #[getter]
    pub fn randomed(&self) -> Option<bool> {
        self.inner.randomed
    }

    #[getter]
    pub fn pred_vict(&self) -> Option<bool> {
        self.inner.pred_vict
    }

    #[getter]
    pub fn stun_duration(&self) -> Option<f32> {
        self.inner.stun_duration
    }

    #[getter]
    pub fn slow_duration(&self) -> Option<f32> {
        self.inner.slow_duration
    }

    #[getter]
    pub fn tracked_death(&self) -> Option<bool> {
        self.inner.tracked_death
    }

    #[getter]
    pub fn greevils_greed_stack(&self) -> Option<u8> {
        self.inner.greevils_greed_stack
    }

    #[getter]
    pub fn tracked_sourcename(&self) -> Option<String> {
        self.inner.tracked_sourcename.clone()
    }

    #[getter]
    pub fn firstblood_claimed(&self) -> Option<bool> {
        self.inner.firstblood_claimed
    }

    #[getter]
    pub fn teamfight_participation(&self) -> Option<f32> {
        self.inner.teamfight_participation
    }

    #[getter]
    pub fn towers_killed(&self) -> Option<u8> {
        self.inner.towers_killed
    }

    #[getter]
    pub fn roshans_killed(&self) -> Option<u8> {
        self.inner.roshans_killed
    }

    #[getter]
    pub fn observers_placed(&self) -> Option<u8> {
        self.inner.observers_placed
    }

    #[getter]
    pub fn draft_order(&self) -> Option<u8> {
        self.inner.draft_order
    }

    #[getter]
    pub fn pick(&self) -> Option<bool> {
        self.inner.pick
    }

    #[getter]
    pub fn draft_active_team(&self) -> Option<u8> {
        self.inner.draft_active_team
    }

    #[getter]
    pub fn draft_extime0(&self) -> Option<u16> {
        self.inner.draft_extime0
    }

    #[getter]
    pub fn draft_extime1(&self) -> Option<u16> {
        self.inner.draft_extime1
    }

    #[getter]
    pub fn networth(&self) -> Option<u32> {
        self.inner.networth
    }

    #[getter]
    pub fn stage(&self) -> Option<u8> {
        self.inner.stage
    }

    pub fn __repr__(&self) -> String {
        serde_json::to_string_pretty(&self.inner).unwrap()
    }

    pub fn __str__(&self) -> String {
        serde_json::to_string_pretty(&self.inner).unwrap()
    }
}

#[pyfunction(name = "parse_replay")]
pub fn parse_replay_py(binary: &[u8]) -> PyResult<Vec<PyEntry>> {
    std::panic::catch_unwind(|| {
        Ok(parse_replay(binary)?
            .into_iter()
            .map(|x| PyEntry { inner: x })
            .collect::<Vec<_>>())
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("Unhandled error\n{e:?}")))
    .and_then(|x| {
        x.map_err(|e: odota_core::Error| PyErr::new::<pyo3::exceptions::PyException, _>(format!("Parser error\n{e}")))
    })
}

#[pymodule]
#[pyo3(name = "odota_py")]
fn odota_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEntry>()?;
    m.add_function(wrap_pyfunction!(parse_replay_py, m)?)?;
    Ok(())
}
