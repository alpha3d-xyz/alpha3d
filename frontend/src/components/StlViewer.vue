<template>
  <div ref="container" class="viewer-container">
    <div v-if="loading" class="viewer-loading">
      {{ $t('viewer.loading') }}
    </div>
    <div v-if="error" class="viewer-error">
      <p><strong>❌ {{ $t('viewer.error') }}</strong></p>
      <p style="font-size: 0.875rem;">{{ error }}</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import * as THREE from 'three';
import { STLLoader } from 'three/examples/jsm/loaders/STLLoader.js';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

const props = defineProps({
  fileUrl: {
    type: String,
    required: true
  },
  autoRotate: {
    type: Boolean,
    default: true
  }
});

const container = ref(null);
const loading = ref(true);
const error = ref(null);

let scene, camera, renderer, controls, mesh, animationId;

const init = () => {
  if (!container.value) return;

  // Scene
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0);
  
  // Camera
  const width = container.value.clientWidth;
  const height = container.value.clientHeight;
  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
  camera.position.set(0, 0, 100);

  // Renderer
  renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(width, height);
  renderer.setPixelRatio(window.devicePixelRatio);
  container.value.appendChild(renderer.domElement);

  // Controls
  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.autoRotate = props.autoRotate;
  controls.autoRotateSpeed = 2.0;

  // Lights
  const ambientLight = new THREE.AmbientLight(0x606060);
  scene.add(ambientLight);

  const directionalLight = new THREE.DirectionalLight(0xffffff);
  directionalLight.position.set(1, 1, 2).normalize();
  scene.add(directionalLight);

  // Grid
  const gridHelper = new THREE.GridHelper(200, 20);
  scene.add(gridHelper);

  // Load STL
  loadStl();

  // Resize handler
  window.addEventListener('resize', onWindowResize);
  
  // Start animation
  animate();
};

const loadStl = () => {
  loading.value = true;
  error.value = null;
  const loader = new STLLoader();
  
  console.log('[StlViewer] Loading STL from URL:', props.fileUrl);
  
  loader.load(
    props.fileUrl,
    (geometry) => {
      console.log('[StlViewer] STL loaded successfully', geometry);
      // Center geometry
      geometry.center();
      geometry.computeBoundingBox();
      
      // Material
      const material = new THREE.MeshPhongMaterial({ 
        color: 0x4f46e5, 
        specular: 0x111111, 
        shininess: 200 
      });
      
      mesh = new THREE.Mesh(geometry, material);
      
      // Rotate to stand up if needed (STLs are often Z-up, Three is Y-up)
      mesh.rotation.x = -Math.PI / 2;
      
      // Adjust camera to fit object
      const boundingBox = geometry.boundingBox;
      const size = new THREE.Vector3();
      boundingBox.getSize(size);
      
      // Raise object to sit on grid (Y-up)
      // size.z is the height because of the rotation
      const objectHeight = size.z;
      mesh.position.y = objectHeight / 2;
      
      scene.add(mesh);

      const maxDim = Math.max(size.x, size.y, size.z);
      const fov = camera.fov * (Math.PI / 180);
      
      // Calculate distance to fit object in view
      // distance = (height / 2) / tan(fov / 2)
      let cameraDistance = Math.abs(maxDim / 2 / Math.tan(fov / 2));
      cameraDistance *= 2.0; // Add some padding
      
      // Update camera far plane if object is large
      if (cameraDistance * 3 > camera.far) {
        camera.far = cameraDistance * 3;
        camera.updateProjectionMatrix();
      }
      
      // Center of the object in world space
      const centerY = objectHeight / 2;
      
      // Position camera at an angle, looking at the center of the object
      camera.position.set(cameraDistance, cameraDistance + centerY, cameraDistance);
      camera.lookAt(0, centerY, 0);
      
      controls.target.set(0, centerY, 0);
      controls.update();

      loading.value = false;
    },
    (xhr) => {
      // Progress
      const progress = (xhr.loaded / xhr.total * 100);
      console.log(`[StlViewer] Loading progress: ${progress.toFixed(0)}%`);
    },
    (err) => {
      console.error('[StlViewer] Error loading STL:', err);
      console.error('[StlViewer] File URL was:', props.fileUrl);
      error.value = `Failed to load STL file: ${err.message || err}`;
      loading.value = false;
    }
  );
};

const onWindowResize = () => {
  if (!container.value || !camera || !renderer) return;
  const width = container.value.clientWidth;
  const height = container.value.clientHeight;
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
  renderer.setSize(width, height);
};

const animate = () => {
  animationId = requestAnimationFrame(animate);
  if (controls) controls.update();
  if (renderer && scene && camera) renderer.render(scene, camera);
};

onMounted(() => {
  init();
});

onBeforeUnmount(() => {
  cancelAnimationFrame(animationId);
  window.removeEventListener('resize', onWindowResize);
  if (renderer && container.value) {
    container.value.removeChild(renderer.domElement);
    renderer.dispose();
  }
  if (mesh) {
    mesh.geometry.dispose();
    mesh.material.dispose();
  }
});

watch(() => props.fileUrl, () => {
  if (mesh) {
    scene.remove(mesh);
    mesh.geometry.dispose();
    mesh.material.dispose();
    mesh = null;
  }
  loadStl();
});
</script>

<style scoped>
.viewer-container {
  width: 100%;
  height: 400px;
  background: #f0f0f0;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.viewer-loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #666;
  font-weight: 500;
}

.viewer-error {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #dc2626;
  text-align: center;
  padding: 1rem;
  max-width: 80%;
}
</style>
