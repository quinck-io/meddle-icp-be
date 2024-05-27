import type { SensorData } from "./types"

/* eslint-disable @typescript-eslint/no-loss-of-precision */
export const DUMMY_IN: SensorData = {
	endpoint: "192.168.101.16",
	variables: [
		{
			sensorId: "AAAA Tensione L1-N",
			value: 236.91445922851562,
			timestamp: 1713166215437,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		},
		{
			sensorId: "Tensione L2-N",
			value: 232.7878875732422,
			timestamp: 1713166215496,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		},
		{
			sensorId: "Tensione L3-N",
			value: 251.20216369628906,
			timestamp: 1713166215576,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		},
		{
			sensorId: "Corrente L1",
			value: 0.06031423434615135,
			timestamp: 1713166215631,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		},
		{
			sensorId: "Corrente L2",
			value: 0.041959624737501144,
			timestamp: 1713166215687,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		},
		{
			sensorId: "Corrente L3",
			value: 0.040930937975645065,
			timestamp: 1713166215753,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		},
		{
			sensorId: "Potenza apparente L1",
			value: 14.275617599487305,
			timestamp: 1713166215806,
			timestampString: "2024-04-15 07:30:15",
			payload: null
		}
	]
}

export const jsonAreaPlaceholder = `Format:
{
	"endpoint": <string>,
	"variables": [
		{
			"sensorId": <string>,
			"value": <number>,
			"timestamp": <bigint>,
			"timestampString": <string>,
			"payload": <unknown>
		}
	]
}
`
