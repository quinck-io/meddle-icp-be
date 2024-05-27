import type { Data, JsonInput } from "$declarations/meddle_backend/meddle_backend.did"

export type SensorRecord = {
	timestamp: number
	sensorId: string
	unitId: string
	value: number
	id: number
}

export type SensorVariable = {
	sensorId: string
	value: number
	timestamp: number
	timestampString: string
	payload?: unknown
}

export type SensorData = {
	endpoint: string
	variables: SensorVariable[]
}

export function encodeRecords(data: Data[]): SensorRecord[] {
	return data.map((r, idx) => ({
		id: idx,
		sensorId: r.sensor_id,
		timestamp: Number(r.timestamp),
		unitId: r.unit_id,
		value: r.value
	}))
}

export function decodeSensorData(data: SensorData): JsonInput {
	return {
		endpoint: data.endpoint,
		variables: data.variables.map((v) => ({
			sensorId: v.sensorId,
			value: v.value,
			timestamp: BigInt(v.timestamp),
			timestampString: v.timestampString
		}))
	}
}
