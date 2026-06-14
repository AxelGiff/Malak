use serde::Serialize;
use sysinfo::{Networks, Disks, System};

#[derive(Debug, Serialize)]
pub struct GlobalData {
    pub nom_machine: String,
    pub systeme_exploitation: String,
    pub version_systeme: String,
    pub uptime: u64,
    pub nombre_processus: usize,
    pub nombre_processeurs: usize,
}

#[derive(Debug, Serialize)]
pub struct MemoryData {
    pub total: f64,
    pub used: f64,
    pub free: f64,
    pub total_swap: f64,
    pub used_swap: f64,
}


#[derive(Debug, Serialize)]
pub struct CpuData {
    pub nom_cpu: String,
    pub usage: f32,
    pub utilisation: f32,
    pub frequence: u64,
    pub brand: String,
    pub id: usize,
    pub total_usage: f32,
}


#[derive(Serialize, Debug)]
pub struct NetworkInfo {
    pub interface: String,
    pub received: u64,
    pub transmitted: u64,
}

#[derive(Debug, Serialize)]
pub struct DiskData {
    pub nom_disque: String,
    pub taille_totale: u64,
    pub espace_libre: u64,
    pub espace_utilise: u64,
    pub file_system: String,
    pub pourcentage_utilisation: f32,
}

#[derive(Debug, Serialize)]
pub struct Metrics {
    pub global_data: GlobalData,
    pub memory: MemoryData,
    pub cpus: Vec<CpuData>,
    pub disks: Vec<DiskData>,
    pub networks: Vec<NetworkInfo>,
}


pub fn get_network_info() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();
    networks
        .iter()
        .map(|(name, data)| NetworkInfo {
            interface: name.clone(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
        })
        .collect()
}


pub fn get_disk_info() -> Vec<DiskData> {
    let disks = Disks::new_with_refreshed_list();
    
    disks
        .iter()
        .map(|disk| DiskData {
            nom_disque: disk.name().to_string_lossy().into_owned(),
            taille_totale: disk.total_space(),
            espace_libre: disk.available_space(),
            file_system: disk.file_system().to_string_lossy().into_owned(),
            espace_utilise: disk.total_space().saturating_sub(disk.available_space()),
            pourcentage_utilisation: if disk.total_space() == 0 {
                0.0
            } else {
                (disk.total_space().saturating_sub(disk.available_space()) as f32
                    / disk.total_space() as f32)
                    * 100.0
            },
        })
        .collect()
}

pub fn get_memory_info(system: &System) -> MemoryData {

    MemoryData {
        total: bytes_to_go(system.total_memory()),
        used: bytes_to_go(system.used_memory()),
        free: bytes_to_go(system.available_memory()),
        total_swap: bytes_to_go(system.total_swap()),
        used_swap: bytes_to_go(system.used_swap()),
    }
}


pub fn get_cpu_info(system: &System) -> Vec<CpuData> {
    system
        .cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| CpuData {
            id: i,
            nom_cpu: cpu.name().to_string(),
            usage: cpu.cpu_usage(),
            utilisation: cpu.cpu_usage(),
            frequence: cpu.frequency(),
            brand: cpu.brand().to_string(),
            total_usage: system.global_cpu_usage(),
        })
        .collect()
}

pub fn get_global_data(system: &System) -> GlobalData {
    GlobalData {
        nom_machine: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        systeme_exploitation: System::name().unwrap_or_else(|| "Unknown".to_string()),
        version_systeme: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        uptime: System::uptime(),
        nombre_processus: system.processes().len(),
        nombre_processeurs: system.cpus().len(),
    }
}

pub fn get_metrics(system: &System) -> Metrics {

    let global = get_global_data(&system);
    let memory = get_memory_info(&system);
    let cpus = get_cpu_info(&system);
    let disks = get_disk_info();
    let networks = get_network_info();

    Metrics {
        global_data: global,
        memory,
        cpus,
        disks,
        networks,
    }
}

pub fn bytes_to_go(bytes: u64) -> f64{
    return bytes as f64/ (1024.0 * 1024.0 * 1024.0);
}