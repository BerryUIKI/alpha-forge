/**
 * Volatility Surface 3D Visualization
 * Uses Three.js for 3D rendering
 */

import { useEffect, useRef } from 'react';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

export interface SurfacePoint {
  strike: number;
  expiration_days: number;
  implied_volatility: number;
}

export interface VolatilitySurface3DProps {
  data: SurfacePoint[];
  symbol: string;
}

export function VolatilitySurface3D({ data, symbol }: VolatilitySurface3DProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<{
    scene: THREE.Scene;
    camera: THREE.PerspectiveCamera;
    renderer: THREE.WebGLRenderer;
    controls: OrbitControls;
    mesh?: THREE.Mesh;
  } | null>(null);

  useEffect(() => {
    if (!containerRef.current || data.length === 0) return;

    // Initialize Three.js scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0xf0f0f0);

    const camera = new THREE.PerspectiveCamera(
      60,
      containerRef.current.clientWidth / containerRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.set(50, 50, 50);

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    containerRef.current.appendChild(renderer.domElement);

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;

    sceneRef.current = { scene, camera, renderer, controls };

    // Add lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(50, 50, 50);
    scene.add(directionalLight);

    // Create surface geometry
    const geometry = new THREE.BufferGeometry();
    const vertices: number[] = [];
    const colors: number[] = [];

    // Normalize data to 3D space
    const minStrike = Math.min(...data.map(d => d.strike));
    const maxStrike = Math.max(...data.map(d => d.strike));
    const minDays = Math.min(...data.map(d => d.expiration_days));
    const maxDays = Math.max(...data.map(d => d.expiration_days));
    const minIV = Math.min(...data.map(d => d.implied_volatility));
    const maxIV = Math.max(...data.map(d => d.implied_volatility));

    for (const point of data) {
      const x = ((point.strike - minStrike) / (maxStrike - minStrike)) * 100;
      const y = ((point.expiration_days - minDays) / (maxDays - minDays)) * 100;
      const z = ((point.implied_volatility - minIV) / (maxIV - minIV)) * 50;

      vertices.push(x, z, y);

      // Color based on IV (green to red)
      const colorValue = (point.implied_volatility - minIV) / (maxIV - minIV);
      const color = new THREE.Color();
      color.setHSL(0.3 - colorValue * 0.3, 0.8, 0.5); // Green to red
      colors.push(color.r, color.g, color.b);
    }

    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));

    // Create points mesh
    const material = new THREE.PointsMaterial({
      size: 2,
      vertexColors: true,
    });
    const points = new THREE.Points(geometry, material);
    scene.add(points);

    // Add axes
    const axesHelper = new THREE.AxesHelper(120);
    scene.add(axesHelper);

    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate);
      controls.update();
      renderer.render(scene, camera);
    };
    animate();

    // Cleanup
    return () => {
      if (containerRef.current && renderer.domElement) {
        containerRef.current.removeChild(renderer.domElement);
      }
      renderer.dispose();
    };
  }, [data]);

  if (data.length === 0) {
    return (
      <div className="flex items-center justify-center h-96 bg-gray-100 rounded">
        <p className="text-gray-500">No volatility surface data available</p>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-4">
        <h3 className="text-lg font-semibold">Volatility Surface - {symbol}</h3>
        <p className="text-sm text-gray-600">
          X: Strike Price | Y: Days to Expiration | Z: Implied Volatility
        </p>
      </div>
      <div ref={containerRef} className="h-96 border rounded-lg" />
    </div>
  );
}