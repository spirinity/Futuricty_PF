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
  lat: number,
  lng: number,
  address: string
): Promise<{ data: LiveabilityData; facilities: Facility[] }> => {
  try {
    const response = await fetch(`${BACKEND_URL}/calculate-score`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ lat, lng }),
    });

    if (!response.ok) {
      throw new Error(`Backend error: ${response.statusText}`);
    }

    const data = await response.json();

    const livabilityData: LiveabilityData = {
      overall: data.scores.overall,
      subscores: {
        services: data.scores.services,
        mobility: data.scores.mobility,
        safety: data.scores.safety,
        environment: data.scores.environment,
      },
      location: {
        address: address, // Use the address passed from frontend
        coordinates: { lng, lat },
      },
      facilityCounts: data.facility_counts,
      nearbyFacilities: data.nearby_facilities,
    };

    // Map the backend facilities to the frontend format
    // The backend returns Facility struct which matches our interface, but we need to ensure types match
    const facilities: Facility[] = (data.facilities || []).map((f: any) => ({
      id: f.id,
      name: f.name,
      category: f.category,
      lng: f.lng,
      lat: f.lat,
      distance: f.distance,
      contribution: f.contribution,
      tags: f.tags
    }));

    return {
      data: livabilityData,
      facilities: facilities,
    };
  } catch (error) {
    console.error("Error calculating score:", error);
    throw error;
  }
};
