import type { Data, JsonInput } from "$declarations/meddle_backend/meddle_backend.did"
import { z } from "zod"

export type SensorRecord = {
	timestamp: number
	sensorId: string
	unitId: string
	value: number
	id: number
}

const sensorVariableSchema = z.object({
	sensorId: z.string(),
	value: z.number(),
	timestamp: z.number(),
	timestampString: z.string(),
	payload: z.unknown().optional()
})
export type SensorVariable = z.infer<typeof sensorVariableSchema>

export const sensorDataSchema = z.object({
	endpoint: z.string(),
	variables: z.array(sensorVariableSchema)
})

export function encodeRecords(data: Data[]): SensorRecord[] {
	return data.map((r, idx) => ({
		id: idx,
		sensorId: r.sensor_id,
		timestamp: Number(r.timestamp),
		unitId: r.unit_id,
		value: r.value
	}))
}

export type SensorData = z.infer<typeof sensorDataSchema>

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
