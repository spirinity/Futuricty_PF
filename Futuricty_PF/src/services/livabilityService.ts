// Livability calculation service using Rust Backend
import { cacheService, cacheUtils } from "./cacheService";

interface Facility {
  id: string;
  name: string;
  category: string;
  lng: number;
  lat: number;
  distance: number;
  contribution: number;
  tags?: any;
}

interface LiveabilityData {
  overall: number;
  subscores: {
    services: number;
    mobility: number;
    safety: number;
    environment: number;
  };
  location: {
    address: string;
    coordinates: { lng: number; lat: number };
  } | null;
  facilityCounts: {
    health: number;
    education: number;
    market: number;
    transport: number;
    walkability: number;
    recreation: number;
    safety: number;
    police: number;
    religious: number;
    accessibility: number;
  };
  nearbyFacilities: string[];
}

const BACKEND_URL = "http://localhost:3000";
const MAX_RETRIES = 3;
const TIMEOUT_MS = 180000; // 3 minutes - increased for large PBF file processing

export const getEmptyLivabilityData = (): LiveabilityData => ({
  overall: 0,
  subscores: {
    services: 0,
    mobility: 0,
    safety: 0,
    environment: 0,
  },
  location: null,
  facilityCounts: {
    health: 0,
    education: 0,
    market: 0,
    transport: 0,
    walkability: 0,
    recreation: 0,
    safety: 0,
    police: 0,
    religious: 0,
    accessibility: 0,
  },
  nearbyFacilities: [],
});

export const calculateLivabilityScore = async (
  locations: Array<{ lat: number; lng: number; address: string }>
): Promise<Array<{ data: LiveabilityData; facilities: Facility[] }>> => {
  for (let retry = 0; retry < MAX_RETRIES; retry++) {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), TIMEOUT_MS);

      const response = await fetch(`${BACKEND_URL}/calculate-score`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          locations: locations.map((l) => ({ lat: l.lat, lng: l.lng })),
        }),
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw new Error(`Backend error: ${response.statusText}`);
      }

      const dataList = await response.json();

      return dataList.map((data: any, index: number) => {
        const livabilityData: LiveabilityData = {
          overall: data.scores.overall,
          subscores: {
            services: data.scores.services,
            mobility: data.scores.mobility,
            safety: data.scores.safety,
            environment: data.scores.environment,
          },
          location: {
            address: locations[index].address,
            coordinates: {
              lat: locations[index].lat,
              lng: locations[index].lng,
            },
          },
          facilityCounts: data.facility_counts,
          nearbyFacilities: data.nearby_facilities,
        };

        const facilities: Facility[] = (data.facilities || []).map(
          (f: any) => ({
            id: f.id,
            name: f.name,
            category: f.category,
            lng: f.lng,
            lat: f.lat,
            distance: f.distance,
            contribution: f.contribution,
            tags: f.tags,
          })
        );

        return {
          data: livabilityData,
          facilities: facilities,
        };
      });
    } catch (error) {
      const isLastRetry = retry === MAX_RETRIES - 1;
      const errorMessage =
        error instanceof Error ? error.message : String(error);

      if (errorMessage === "The operation was aborted.") {
        console.warn(`Request timeout (retry ${retry + 1}/${MAX_RETRIES})`);
      } else {
        console.warn(
          `Error calculating score (retry ${retry + 1}/${MAX_RETRIES}):`,
          error
        );
      }

      if (isLastRetry) {
        throw new Error(
          `Failed to calculate livability score after ${MAX_RETRIES} attempts. ${errorMessage}`
        );
      }

      // Exponential backoff: 1s, 2s, 4s
      const delayMs = 1000 * Math.pow(2, retry);
      console.log(`Retrying in ${delayMs}ms...`);
      await new Promise((resolve) => setTimeout(resolve, delayMs));
    }
  }

  throw new Error("Unexpected error in calculateLivabilityScore");
};
