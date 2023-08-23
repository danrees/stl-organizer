import * as three from "three";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";

const createCamera = (el: HTMLCanvasElement) => {
  const camera = new three.PerspectiveCamera(
    70,
    el.clientWidth / el.clientHeight,
    1.0,
    1000,
  );

  camera.position.z = 5;
  return camera;
};

const createScene = (
  el: HTMLCanvasElement,
  window: Window,
  path: ArrayBuffer,
) => {
  const scene = buildScene();

  const camera = createCamera(el);
  const renderer = buildRenderer(el, window, path, false, camera, scene);

  let controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.rotateSpeed = 0.05;
  controls.dampingFactor = 0.1;
  controls.enableZoom = true;
  controls.autoRotate = true;
  controls.autoRotateSpeed = 0.75;

  const animate = () => {
    requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
  };

  return animate;
};

const buildScene = () => {
  const scene = new three.Scene();
  scene.add(new three.HemisphereLight(0xffffff, 1.5));

  return scene;
};

const buildRenderer = (
  el: HTMLCanvasElement,
  window: Window,
  path: ArrayBuffer,
  preserveDrawingBuffer: boolean,
  camera: three.PerspectiveCamera,
  scene: three.Scene,
) => {
  // const camera = createCamera(el);
  // const scene = buildScene();

  const renderer = new three.WebGLRenderer({
    antialias: true,
    canvas: el,
    preserveDrawingBuffer: preserveDrawingBuffer,
  });
  renderer.setSize(window.innerWidth, window.innerHeight);

  const loader = new STLLoader();

  const geometry = loader.parse(path);

  const material = new three.MeshPhongMaterial({
    color: 0xff5533,
    specular: 100,
    shininess: 100,
  });

  const mesh = new three.Mesh(geometry, material);
  scene.add(mesh);
  const middle = new three.Vector3();
  geometry.computeBoundingBox();
  geometry?.boundingBox?.getCenter(middle);
  mesh.geometry.applyMatrix4(
    new three.Matrix4().makeTranslation(-middle.x, -middle.y, -middle.z),
  );

  if (geometry.boundingBox) {
    const largestDimension = Math.max(
      geometry.boundingBox.max.x,
      geometry.boundingBox.max.y,
      geometry.boundingBox.max.z,
    );

    camera.position.z = largestDimension * 1.5;
  }
  return renderer;
};

const saveImage = (
  el: HTMLCanvasElement,
  window: Window,
  path: ArrayBuffer,
  callBack: BlobCallback,
) => {
  const scene = buildScene();
  const camera = createCamera(el);
  const renderer = buildRenderer(el, window, path, true, camera, scene);

  renderer.render(scene, camera);
  el.toBlob(callBack, "img/png");
};

export { createScene, saveImage };
