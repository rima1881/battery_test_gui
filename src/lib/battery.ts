import { ref, computed } from 'vue'

export enum BatteryBenchState  {
	STANDBY,
	CHARGE,
	DISCHARGE
}

export enum CompletionStatus {
	SUCCESS,
	FAIL,
	IN_PROGRESS
}

export interface BatteryBench {
	id: number;
	port: String;
	temperature: number;
	battery_temperature: number;
	electronic_load_temperature: number;
	voltage: number;
	current: number;
	state: BatteryBenchState;
	status: CompletionStatus;
	start_date: Date;
}

export function useBatteryManager() {
	const batteries = ref<BatteryBench[]>([]);
	
	const batteries_voltages = computed(() => {
		return []
	})
	
	const batteries_temperatures = computed(() => {
		return []
	})
	
	const batteries_currents = computed(() => {
		return []
	})
	
	const battery_benches_temperatures = computed(() => {
		return []
	})
	
	const bench_loads_temperatures = computed(() => {
		return []
	})
	
	return {
		batteries,
		batteries_voltages,
		batteries_currents,
		batteries_temperatures,
		battery_benches_temperatures,
		bench_loads_temperatures
	}
}
