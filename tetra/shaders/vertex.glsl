#version 460

//-------// IN //-------//

layout(set = 0, binding = 0) uniform CameraMatrices {
    mat4 view;
    mat4 projection;
};

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 model_position;
layout(location = 2) in float model_scale;
layout(location = 3) in vec4 model_orientation;

//-------// OUT //-------//

//------// CONST //------//

//////////////////////////////////////////////////////
// Functions (from chatGPT + redesigned)
//////////////////////////////////////////////////////

mat4 quaternion_to_rotation_matrix(vec4 q) {
    // extract components
    float qw = q.w, qx = q.x, qy = q.y, qz = q.z;
    
    // compute
    mat4 rotMat = mat4(
        1.0 - 2.0 * (qy * qy + qz * qz),  2.0 * (qx * qy - qw * qz),  2.0 * (qx * qz + qw * qy),  0.0,
        2.0 * (qx * qy + qw * qz),  1.0 - 2.0 * (qx * qx + qz * qz),  2.0 * (qy * qz - qw * qx),  0.0,
        2.0 * (qx * qz - qw * qy),  2.0 * (qy * qz + qw * qx),  1.0 - 2.0 * (qx * qx + qy * qy),  0.0,
        0.0,  0.0,  0.0,  1.0
    );

    return rotMat;
}

mat4 model_matrix_from(vec3 position, float scale, vec4 orientation) {
    // scaling matrix
    mat4 scaleMat = mat4(1.0);
    scaleMat[0][0] = scale; 
    scaleMat[1][1] = scale;
    scaleMat[2][2] = scale;
    
    // rotation matrix
    mat4 rotationMat = quaternion_to_rotation_matrix(orientation);
    
    // translation matrix
    mat4 translationMat = mat4(1.0);
    translationMat[3][0] = position.x;
    translationMat[3][1] = position.y;
    translationMat[3][2] = position.z;
    
    return translationMat * rotationMat * scaleMat;
}

//////////////////////////////////////////////////////
// Main
//////////////////////////////////////////////////////

void main() {
    mat4 model = model_matrix_from(model_position, model_scale, model_orientation);
    gl_Position = projection * view * model * vec4(position, 1.0);
}
