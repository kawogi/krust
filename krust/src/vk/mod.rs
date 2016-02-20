extern crate libc;

pub mod types;
pub mod consts;
pub mod enums;
pub mod flags;
pub mod structs;
pub mod fns;

//use libc::*;

// /
// / File: vk_platform.h
// /
// *
//** Copyright (c) 2014-2015 The Khronos Group Inc.
//**
//** Permission is hereby granted, free of charge, to any person obtaining a
//** copy of this software and/or associated documentation files (the
//** "Materials"), to deal in the Materials without restriction, including
//** without limitation the rights to use, copy, modify, merge, publish,
//** distribute, sublicense, and/or sell copies of the Materials, and to
//** permit persons to whom the Materials are furnished to do so, subject to
//** the following conditions:
//**
//** The above copyright notice and this permission notice shall be included
//** in all copies or substantial portions of the Materials.
//**
//** THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
//** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
//** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
//** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
//** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
//** MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.
//*/
//
//
//#ifndef __VK_PLATFORM_H__
//#define __VK_PLATFORM_H__
//
//#ifdef __cplusplus
//extern "C"
//{
//#endif // __cplusplus
//
// *
//***************************************************************************************************
//* Platform-specific directives and type declarations
//***************************************************************************************************
//*/
//
// * Platform-specific calling convention macros.
// *
// * Platforms should define these so that Vulkan clients call Vulkan commands
// * with the same calling conventions that the Vulkan implementation expects.
// *
// * VKAPI_ATTR - Placed before the return type in function declarations.
// * Useful for C++11 and GCC/Clang-style function attribute syntax.
// * VKAPI_CALL - Placed after the return type in function declarations.
// * Useful for MSVC-style calling convention syntax.
// * VKAPI_PTR - Placed between the '(' and '*' in function pointer types.
// *
// * Function declaration: VKAPI_ATTR void VKAPI_CALL vkCommand(void);
// * Function pointer type: typedef void (VKAPI_PTR *PFN_vkCommand)(void);
// */
//#if defined(_WIN32)
//    // On Windows, Vulkan commands use the stdcall convention
//    #define VKAPI_ATTR
//    #define VKAPI_CALL __stdcall
//    #define VKAPI_PTR  VKAPI_CALL
//#elif defined(__ANDROID__) && defined(__ARM_EABI__) && !defined(__ARM_ARCH_7A__)
//    // Android does not support Vulkan in native code using the "armeabi" ABI.
//    #error "Vulkan requires the 'armeabi-v7a' or 'armeabi-v7a-hard' ABI on 32-bit ARM CPUs"
//#elif defined(__ANDROID__) && defined(__ARM_ARCH_7A__)
//    // On Android/ARMv7a, Vulkan functions use the armeabi-v7a-hard calling
//    // convention, even if the application's native code is compiled with the
//    // armeabi-v7a calling convention.
//    #define VKAPI_ATTR __attribute__((pcs("aapcs-vfp")))
//    #define VKAPI_CALL
//    #define VKAPI_PTR  VKAPI_ATTR
//#else
//    // On other platforms, use the default calling convention
//    #define VKAPI_ATTR
//    #define VKAPI_CALL
//    #define VKAPI_PTR
//#endif
//
//#include <stddef.h>
//
//#if !defined(VK_NO_STDINT_H)
//    #if defined(_MSC_VER) && (_MSC_VER < 1600)
//        typedef signed   __int8  int8_t;
//        typedef unsigned __int8  u8;
//        typedef signed   __int16 int16_t;
//        typedef unsigned __int16 uint16_t;
//        typedef signed   __int32 int32_t;
//        typedef unsigned __int32 uint32_t;
//        typedef signed   __int64 int64_t;
//        typedef unsigned __int64 uint64_t;
//    #else
//        #include <stdint.h>
//    #endif
//#endif // !defined(VK_NO_STDINT_H)
//
//#ifdef __cplusplus
//} // extern "C"
//#endif // __cplusplus
//
// // Platform-specific headers required by platform window system extensions.
// // These are enabled prior to #including "vulkan.h". The same enable then
// // controls inclusion of the extension interfaces in vulkan.h.
//
//#ifdef VK_USE_PLATFORM_ANDROID_KHR
//#include <android/native_window.h>
//#endif
//
//#ifdef VK_USE_PLATFORM_MIR_KHR
//#include <mir_toolkit/client_types.h>
//#endif
//
//#ifdef VK_USE_PLATFORM_WAYLAND_KHR
//#include <wayland-client.h>
//#endif
//
//#ifdef VK_USE_PLATFORM_WIN32_KHR
//#include <windows.h>
//#endif
//
//#ifdef VK_USE_PLATFORM_XLIB_KHR
//#include <X11/Xlib.h>
//#endif
//
//#ifdef VK_USE_PLATFORM_XCB_KHR
//#include <xcb/xcb.h>
//#endif
//
//#endif // __VK_PLATFORM_H__




/*
** Copyright (c) 2015-2016 The Khronos Group Inc.
**
** Permission is hereby granted, free of charge, to any person obtaining a
** copy of this software and/or associated documentation files (the
** "Materials"), to deal in the Materials without restriction, including
** without limitation the rights to use, copy, modify, merge, publish,
** distribute, sublicense, and/or sell copies of the Materials, and to
** permit persons to whom the Materials are furnished to do so, subject to
** the following conditions:
**
** The above copyright notice and this permission notice shall be included
** in all copies or substantial portions of the Materials.
**
** THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
** MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.
*/

/*
** This header is generated from the Khronos Vulkan XML API Registry.
**
*/


//#define VK_VERSION_1_0 1
//
//include "vk_platform.h"
//
//#define VK_MAKE_VERSION(major, minor, patch) (((major) << 22) | ((minor) << 12) | (patch))
//
// // Vulkan API version supported by this file
//#define VK_API_VERSION VK_MAKE_VERSION(1, 0, 3)
//
//#define VK_VERSION_MAJOR(version) ((u32)(version) >> 22)
//#define VK_VERSION_MINOR(version) (((u32)(version) >> 12) & 0x3ff)
//#define VK_VERSION_PATCH(version) ((u32)(version) & 0xfff)
//
//#define VK_NULL_HANDLE 0
//    

// include consts.rs
// include enums.rs
// include flags.rs
// include structs.rs
// include types.rs

//#ifndef VK_NO_PROTOTYPES
// include fns.rs
//#endif


//#define VK_KHR_surface 1
//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSurfaceKHR)
//
//#define VK_KHR_SURFACE_SPEC_VERSION       25
//#define VK_KHR_SURFACE_EXTENSION_NAME     "VK_KHR_surface"
//
//
//typedef enum VkColorSpaceKHR {
//    VK_COLORSPACE_SRGB_NONLINEAR_KHR = 0,
//    VK_COLORSPACE_BEGIN_RANGE = VK_COLORSPACE_SRGB_NONLINEAR_KHR,
//    VK_COLORSPACE_END_RANGE = VK_COLORSPACE_SRGB_NONLINEAR_KHR,
//    VK_COLORSPACE_RANGE_SIZE = (VK_COLORSPACE_SRGB_NONLINEAR_KHR - VK_COLORSPACE_SRGB_NONLINEAR_KHR + 1),
//    VK_COLORSPACE_MAX_ENUM = 0x7FFFFFFF
//} VkColorSpaceKHR;
//
//typedef enum VkPresentModeKHR {
//    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
//    VK_PRESENT_MODE_MAILBOX_KHR = 1,
//    VK_PRESENT_MODE_FIFO_KHR = 2,
//    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
//    VK_PRESENT_MODE_BEGIN_RANGE = VK_PRESENT_MODE_IMMEDIATE_KHR,
//    VK_PRESENT_MODE_END_RANGE = VK_PRESENT_MODE_FIFO_RELAXED_KHR,
//    VK_PRESENT_MODE_RANGE_SIZE = (VK_PRESENT_MODE_FIFO_RELAXED_KHR - VK_PRESENT_MODE_IMMEDIATE_KHR + 1),
//    VK_PRESENT_MODE_MAX_ENUM = 0x7FFFFFFF
//} VkPresentModeKHR;
//
//
//typedef enum VkSurfaceTransformFlagBitsKHR {
//    VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
//    VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
//    VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
//    VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
//    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
//    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
//    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
//    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
//    VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100,
//} VkSurfaceTransformFlagBitsKHR;
//typedef VkFlags VkSurfaceTransformFlagsKHR;
//
//typedef enum VkCompositeAlphaFlagBitsKHR {
//    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
//    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
//    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
//    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
//} VkCompositeAlphaFlagBitsKHR;
//typedef VkFlags VkCompositeAlphaFlagsKHR;
//
//typedef struct VkSurfaceCapabilitiesKHR {
//    u32 minImageCount;
//    u32 maxImageCount;
//    VkExtent2D                       currentExtent;
//    VkExtent2D                       minImageExtent;
//    VkExtent2D                       maxImageExtent;
//    u32 maxImageArrayLayers;
//    VkSurfaceTransformFlagsKHR supportedTransforms;
//    VkSurfaceTransformFlagBitsKHR currentTransform;
//    VkCompositeAlphaFlagsKHR supportedCompositeAlpha;
//    VkImageUsageFlags supportedUsageFlags;
//} VkSurfaceCapabilitiesKHR;
//
//typedef struct VkSurfaceFormatKHR {
//    VkFormat format;
//    VkColorSpaceKHR colorSpace;
//} VkSurfaceFormatKHR;
//
//
//typedef void (*PFN_vkDestroySurfaceKHR)(VkInstance instance, VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceSupportKHR)(VkPhysicalDevice physicalDevice, u32 queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported);
//typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);
//typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, u32* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats);
//typedef VkResult (*PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, u32* pPresentModeCount, VkPresentModeKHR* pPresentModes);
//
//#ifndef VK_NO_PROTOTYPES
//void vkDestroySurfaceKHR(
//    VkInstance instance,
//    VkSurfaceKHR surface,
//    const VkAllocationCallbacks* pAllocator);
//
//VkResult vkGetPhysicalDeviceSurfaceSupportKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 queueFamilyIndex,
//    VkSurfaceKHR surface,
//    VkBool32* pSupported);
//
//VkResult vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
//    VkPhysicalDevice physicalDevice,
//    VkSurfaceKHR surface,
//    VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);
//
//VkResult vkGetPhysicalDeviceSurfaceFormatsKHR(
//    VkPhysicalDevice physicalDevice,
//    VkSurfaceKHR surface,
//    u32* pSurfaceFormatCount,
//    VkSurfaceFormatKHR* pSurfaceFormats);
//
//VkResult vkGetPhysicalDeviceSurfacePresentModesKHR(
//    VkPhysicalDevice physicalDevice,
//    VkSurfaceKHR surface,
//    u32* pPresentModeCount,
//    VkPresentModeKHR* pPresentModes);
//#endif
//
//#define VK_KHR_swapchain 1
//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSwapchainKHR)
//
//#define VK_KHR_SWAPCHAIN_SPEC_VERSION     67
//#define VK_KHR_SWAPCHAIN_EXTENSION_NAME   "VK_KHR_swapchain"
//
//typedef VkFlags VkSwapchainCreateFlagsKHR;
//
//typedef struct VkSwapchainCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkSwapchainCreateFlagsKHR flags;
//    VkSurfaceKHR surface;
//    u32 minImageCount;
//    VkFormat imageFormat;
//    VkColorSpaceKHR imageColorSpace;
//    VkExtent2D                       imageExtent;
//    u32 imageArrayLayers;
//    VkImageUsageFlags imageUsage;
//    VkSharingMode imageSharingMode;
//    u32 queueFamilyIndexCount;
//    const u32* pQueueFamilyIndices;
//    VkSurfaceTransformFlagBitsKHR preTransform;
//    VkCompositeAlphaFlagBitsKHR compositeAlpha;
//    VkPresentModeKHR presentMode;
//    VkBool32 clipped;
//    VkSwapchainKHR oldSwapchain;
//} VkSwapchainCreateInfoKHR;
//
//typedef struct VkPresentInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    u32 waitSemaphoreCount;
//    const VkSemaphore* pWaitSemaphores;
//    u32 swapchainCount;
//    const VkSwapchainKHR* pSwapchains;
//    const u32* pImageIndices;
//    VkResult* pResults;
//} VkPresentInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateSwapchainKHR)(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain);
//typedef void (*PFN_vkDestroySwapchainKHR)(VkDevice device, VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkGetSwapchainImagesKHR)(VkDevice device, VkSwapchainKHR swapchain, u32* pSwapchainImageCount, VkImage* pSwapchainImages);
//typedef VkResult (*PFN_vkAcquireNextImageKHR)(VkDevice device, VkSwapchainKHR swapchain, u64 timeout, VkSemaphore semaphore, VkFence fence, u32* pImageIndex);
//typedef VkResult (*PFN_vkQueuePresentKHR)(VkQueue queue, const VkPresentInfoKHR* pPresentInfo);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateSwapchainKHR(
//    VkDevice device,
//    const VkSwapchainCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSwapchainKHR* pSwapchain);
//
//void vkDestroySwapchainKHR(
//    VkDevice device,
//    VkSwapchainKHR swapchain,
//    const VkAllocationCallbacks* pAllocator);
//
//VkResult vkGetSwapchainImagesKHR(
//    VkDevice device,
//    VkSwapchainKHR swapchain,
//    u32* pSwapchainImageCount,
//    VkImage* pSwapchainImages);
//
//VkResult vkAcquireNextImageKHR(
//    VkDevice device,
//    VkSwapchainKHR swapchain,
//    u64 timeout,
//    VkSemaphore                                 semaphore,
//    VkFence                                     fence,
//    u32* pImageIndex);
//
//VkResult vkQueuePresentKHR(
//    VkQueue                                     queue,
//    const VkPresentInfoKHR* pPresentInfo);
//#endif
//
//#define VK_KHR_display 1
//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDisplayKHR)
//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDisplayModeKHR)
//
//#define VK_KHR_DISPLAY_SPEC_VERSION       21
//#define VK_KHR_DISPLAY_EXTENSION_NAME     "VK_KHR_display"
//
//
//typedef enum VkDisplayPlaneAlphaFlagBitsKHR {
//    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
//    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002,
//    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004,
//    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008,
//} VkDisplayPlaneAlphaFlagBitsKHR;
//typedef VkFlags VkDisplayModeCreateFlagsKHR;
//typedef VkFlags VkDisplayPlaneAlphaFlagsKHR;
//typedef VkFlags VkDisplaySurfaceCreateFlagsKHR;
//
//typedef struct VkDisplayPropertiesKHR {
//    VkDisplayKHR display;
//    const char* displayName;
//    VkExtent2D                    physicalDimensions;
//    VkExtent2D                    physicalResolution;
//    VkSurfaceTransformFlagsKHR supportedTransforms;
//    VkBool32 planeReorderPossible;
//    VkBool32 persistentContent;
//} VkDisplayPropertiesKHR;
//
//typedef struct VkDisplayModeParametersKHR {
//    VkExtent2D    visibleRegion;
//    u32 refreshRate;
//} VkDisplayModeParametersKHR;
//
//typedef struct VkDisplayModePropertiesKHR {
//    VkDisplayModeKHR displayMode;
//    VkDisplayModeParametersKHR parameters;
//} VkDisplayModePropertiesKHR;
//
//typedef struct VkDisplayModeCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkDisplayModeCreateFlagsKHR flags;
//    VkDisplayModeParametersKHR parameters;
//} VkDisplayModeCreateInfoKHR;
//
//typedef struct VkDisplayPlaneCapabilitiesKHR {
//    VkDisplayPlaneAlphaFlagsKHR supportedAlpha;
//    VkOffset2D                     minSrcPosition;
//    VkOffset2D                     maxSrcPosition;
//    VkExtent2D                     minSrcExtent;
//    VkExtent2D                     maxSrcExtent;
//    VkOffset2D                     minDstPosition;
//    VkOffset2D                     maxDstPosition;
//    VkExtent2D                     minDstExtent;
//    VkExtent2D                     maxDstExtent;
//} VkDisplayPlaneCapabilitiesKHR;
//
//typedef struct VkDisplayPlanePropertiesKHR {
//    VkDisplayKHR currentDisplay;
//    u32 currentStackIndex;
//} VkDisplayPlanePropertiesKHR;
//
//typedef struct VkDisplaySurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkDisplaySurfaceCreateFlagsKHR flags;
//    VkDisplayModeKHR displayMode;
//    u32 planeIndex;
//    u32 planeStackIndex;
//    VkSurfaceTransformFlagBitsKHR transform;
//    float globalAlpha;
//    VkDisplayPlaneAlphaFlagBitsKHR alphaMode;
//    VkExtent2D                        imageExtent;
//} VkDisplaySurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkGetPhysicalDeviceDisplayPropertiesKHR)(VkPhysicalDevice physicalDevice, u32* pPropertyCount, VkDisplayPropertiesKHR* pProperties);
//typedef VkResult (*PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR)(VkPhysicalDevice physicalDevice, u32* pPropertyCount, VkDisplayPlanePropertiesKHR* pProperties);
//typedef VkResult (*PFN_vkGetDisplayPlaneSupportedDisplaysKHR)(VkPhysicalDevice physicalDevice, u32 planeIndex, u32* pDisplayCount, VkDisplayKHR* pDisplays);
//typedef VkResult (*PFN_vkGetDisplayModePropertiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, u32* pPropertyCount, VkDisplayModePropertiesKHR* pProperties);
//typedef VkResult (*PFN_vkCreateDisplayModeKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, const VkDisplayModeCreateInfoKHR*pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDisplayModeKHR* pMode);
//typedef VkResult (*PFN_vkGetDisplayPlaneCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayModeKHR mode, u32 planeIndex, VkDisplayPlaneCapabilitiesKHR* pCapabilities);
//typedef VkResult (*PFN_vkCreateDisplayPlaneSurfaceKHR)(VkInstance instance, const VkDisplaySurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkGetPhysicalDeviceDisplayPropertiesKHR(
//    VkPhysicalDevice physicalDevice,
//    u32* pPropertyCount,
//    VkDisplayPropertiesKHR* pProperties);
//
//VkResult vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
//    VkPhysicalDevice physicalDevice,
//    u32* pPropertyCount,
//    VkDisplayPlanePropertiesKHR* pProperties);
//
//VkResult vkGetDisplayPlaneSupportedDisplaysKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 planeIndex,
//    u32* pDisplayCount,
//    VkDisplayKHR* pDisplays);
//
//VkResult vkGetDisplayModePropertiesKHR(
//    VkPhysicalDevice physicalDevice,
//    VkDisplayKHR display,
//    u32* pPropertyCount,
//    VkDisplayModePropertiesKHR* pProperties);
//
//VkResult vkCreateDisplayModeKHR(
//    VkPhysicalDevice physicalDevice,
//    VkDisplayKHR display,
//    const VkDisplayModeCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkDisplayModeKHR* pMode);
//
//VkResult vkGetDisplayPlaneCapabilitiesKHR(
//    VkPhysicalDevice physicalDevice,
//    VkDisplayModeKHR mode,
//    u32 planeIndex,
//    VkDisplayPlaneCapabilitiesKHR* pCapabilities);
//
//VkResult vkCreateDisplayPlaneSurfaceKHR(
//    VkInstance instance,
//    const VkDisplaySurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//#endif
//
//#define VK_KHR_display_swapchain 1
//#define VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION 9
//#define VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME "VK_KHR_display_swapchain"
//
//typedef struct VkDisplayPresentInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkRect2D srcRect;
//    VkRect2D dstRect;
//    VkBool32 persistent;
//} VkDisplayPresentInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateSharedSwapchainsKHR)(VkDevice device, u32 swapchainCount, const VkSwapchainCreateInfoKHR* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchains);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateSharedSwapchainsKHR(
//    VkDevice device,
//    u32 swapchainCount,
//    const VkSwapchainCreateInfoKHR* pCreateInfos,
//    const VkAllocationCallbacks* pAllocator,
//    VkSwapchainKHR* pSwapchains);
//#endif
//
//#ifdef VK_USE_PLATFORM_XLIB_KHR
//#define VK_KHR_xlib_surface 1
//#include <X11/Xlib.h>
//
//#define VK_KHR_XLIB_SURFACE_SPEC_VERSION  6
//#define VK_KHR_XLIB_SURFACE_EXTENSION_NAME "VK_KHR_xlib_surface"
//
//typedef VkFlags VkXlibSurfaceCreateFlagsKHR;
//
//typedef struct VkXlibSurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkXlibSurfaceCreateFlagsKHR flags;
//    Display* dpy;
//    Window                         window;
//} VkXlibSurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateXlibSurfaceKHR)(VkInstance instance, const VkXlibSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//typedef VkBool32 (*PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR)(VkPhysicalDevice physicalDevice, u32 queueFamilyIndex, Display* dpy, VisualID visualID);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateXlibSurfaceKHR(
//    VkInstance instance,
//    const VkXlibSurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//
//VkBool32 vkGetPhysicalDeviceXlibPresentationSupportKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 queueFamilyIndex,
//    Display* dpy,
//    VisualID                                    visualID);
//#endif
//#endif /* VK_USE_PLATFORM_XLIB_KHR */
//
//#ifdef VK_USE_PLATFORM_XCB_KHR
//#define VK_KHR_xcb_surface 1
//#include <xcb/xcb.h>
//
//#define VK_KHR_XCB_SURFACE_SPEC_VERSION   6
//#define VK_KHR_XCB_SURFACE_EXTENSION_NAME "VK_KHR_xcb_surface"
//
//typedef VkFlags VkXcbSurfaceCreateFlagsKHR;
//
//typedef struct VkXcbSurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkXcbSurfaceCreateFlagsKHR flags;
//    xcb_connection_t* connection;
//    xcb_window_t                  window;
//} VkXcbSurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateXcbSurfaceKHR)(VkInstance instance, const VkXcbSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//typedef VkBool32 (*PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR)(VkPhysicalDevice physicalDevice, u32 queueFamilyIndex, xcb_connection_t* connection, xcb_visualid_t visual_id);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateXcbSurfaceKHR(
//    VkInstance instance,
//    const VkXcbSurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//
//VkBool32 vkGetPhysicalDeviceXcbPresentationSupportKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 queueFamilyIndex,
//    xcb_connection_t* connection,
//    xcb_visualid_t                              visual_id);
//#endif
//#endif /* VK_USE_PLATFORM_XCB_KHR */
//
//#ifdef VK_USE_PLATFORM_WAYLAND_KHR
//#define VK_KHR_wayland_surface 1
//#include <wayland-client.h>
//
//#define VK_KHR_WAYLAND_SURFACE_SPEC_VERSION 5
//#define VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME "VK_KHR_wayland_surface"
//
//typedef VkFlags VkWaylandSurfaceCreateFlagsKHR;
//
//typedef struct VkWaylandSurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkWaylandSurfaceCreateFlagsKHR flags;
//    struct wl_display* display;
//    struct wl_surface* surface;
//} VkWaylandSurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateWaylandSurfaceKHR)(VkInstance instance, const VkWaylandSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//typedef VkBool32 (*PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR)(VkPhysicalDevice physicalDevice, u32 queueFamilyIndex, struct wl_display* display);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateWaylandSurfaceKHR(
//    VkInstance instance,
//    const VkWaylandSurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//
//VkBool32 vkGetPhysicalDeviceWaylandPresentationSupportKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 queueFamilyIndex,
//    struct wl_display* display);
//#endif
//#endif /* VK_USE_PLATFORM_WAYLAND_KHR */
//
//#ifdef VK_USE_PLATFORM_MIR_KHR
//#define VK_KHR_mir_surface 1
//#include <mir_toolkit/client_types.h>
//
//#define VK_KHR_MIR_SURFACE_SPEC_VERSION   4
//#define VK_KHR_MIR_SURFACE_EXTENSION_NAME "VK_KHR_mir_surface"
//
//typedef VkFlags VkMirSurfaceCreateFlagsKHR;
//
//typedef struct VkMirSurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkMirSurfaceCreateFlagsKHR flags;
//    MirConnection* connection;
//    MirSurface* mirSurface;
//} VkMirSurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateMirSurfaceKHR)(VkInstance instance, const VkMirSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//typedef VkBool32 (*PFN_vkGetPhysicalDeviceMirPresentationSupportKHR)(VkPhysicalDevice physicalDevice, u32 queueFamilyIndex, MirConnection* connection);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateMirSurfaceKHR(
//    VkInstance instance,
//    const VkMirSurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//
//VkBool32 vkGetPhysicalDeviceMirPresentationSupportKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 queueFamilyIndex,
//    MirConnection* connection);
//#endif
//#endif /* VK_USE_PLATFORM_MIR_KHR */
//
//#ifdef VK_USE_PLATFORM_ANDROID_KHR
//#define VK_KHR_android_surface 1
//#include <android/native_window.h>
//
//#define VK_KHR_ANDROID_SURFACE_SPEC_VERSION 6
//#define VK_KHR_ANDROID_SURFACE_EXTENSION_NAME "VK_KHR_android_surface"
//
//typedef VkFlags VkAndroidSurfaceCreateFlagsKHR;
//
//typedef struct VkAndroidSurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkAndroidSurfaceCreateFlagsKHR flags;
//    ANativeWindow* window;
//} VkAndroidSurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateAndroidSurfaceKHR)(VkInstance instance, const VkAndroidSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateAndroidSurfaceKHR(
//    VkInstance instance,
//    const VkAndroidSurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//#endif
//#endif /* VK_USE_PLATFORM_ANDROID_KHR */
//
//#ifdef VK_USE_PLATFORM_WIN32_KHR
//#define VK_KHR_win32_surface 1
//#include <windows.h>
//
//#define VK_KHR_WIN32_SURFACE_SPEC_VERSION 5
//#define VK_KHR_WIN32_SURFACE_EXTENSION_NAME "VK_KHR_win32_surface"
//
//typedef VkFlags VkWin32SurfaceCreateFlagsKHR;
//
//typedef struct VkWin32SurfaceCreateInfoKHR {
//    VkStructureType sType;
//    const void* pNext;
//    VkWin32SurfaceCreateFlagsKHR flags;
//    HINSTANCE hinstance;
//    HWND hwnd;
//} VkWin32SurfaceCreateInfoKHR;
//
//
//typedef VkResult (*PFN_vkCreateWin32SurfaceKHR)(VkInstance instance, const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
//typedef VkBool32 (*PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR)(VkPhysicalDevice physicalDevice, u32 queueFamilyIndex);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateWin32SurfaceKHR(
//    VkInstance instance,
//    const VkWin32SurfaceCreateInfoKHR* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkSurfaceKHR* pSurface);
//
//VkBool32 vkGetPhysicalDeviceWin32PresentationSupportKHR(
//    VkPhysicalDevice physicalDevice,
//    u32 queueFamilyIndex);
//#endif
//#endif /* VK_USE_PLATFORM_WIN32_KHR */
//
//#define VK_EXT_debug_report 1
//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDebugReportCallbackEXT)
//
//#define VK_EXT_DEBUG_REPORT_SPEC_VERSION  1
//#define VK_EXT_DEBUG_REPORT_EXTENSION_NAME "VK_EXT_debug_report"
//
//
//typedef enum VkDebugReportObjectTypeEXT {
//    VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0,
//    VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1,
//    VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2,
//    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3,
//    VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4,
//    VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5,
//    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6,
//    VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7,
//    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8,
//    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9,
//    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10,
//    VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11,
//    VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12,
//    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13,
//    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14,
//    VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15,
//    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16,
//    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17,
//    VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18,
//    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19,
//    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20,
//    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21,
//    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22,
//    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23,
//    VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24,
//    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25,
//    VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26,
//    VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27,
//    VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT = 28,
//} VkDebugReportObjectTypeEXT;
//
//typedef enum VkDebugReportErrorEXT {
//    VK_DEBUG_REPORT_ERROR_NONE_EXT = 0,
//    VK_DEBUG_REPORT_ERROR_CALLBACK_REF_EXT = 1,
//} VkDebugReportErrorEXT;
//
//
//typedef enum VkDebugReportFlagBitsEXT {
//    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,
//    VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,
//    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,
//    VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,
//    VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010,
//} VkDebugReportFlagBitsEXT;
//typedef VkFlags VkDebugReportFlagsEXT;
//
//typedef VkBool32 (*PFN_vkDebugReportCallbackEXT)(
//    VkDebugReportFlagsEXT flags,
//    VkDebugReportObjectTypeEXT objectType,
//    u64 object,
//    usize location,
//    i32 messageCode,
//    const char* pLayerPrefix,
//    const char* pMessage,
//    void* pUserData);
//
//
//typedef struct VkDebugReportCallbackCreateInfoEXT {
//    VkStructureType sType;
//    const void* pNext;
//    VkDebugReportFlagsEXT flags;
//    PFN_vkDebugReportCallbackEXT pfnCallback;
//    void* pUserData;
//} VkDebugReportCallbackCreateInfoEXT;
//
//
//typedef VkResult (*PFN_vkCreateDebugReportCallbackEXT)(VkInstance instance, const VkDebugReportCallbackCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugReportCallbackEXT* pCallback);
//typedef void (*PFN_vkDestroyDebugReportCallbackEXT)(VkInstance instance, VkDebugReportCallbackEXT callback, const VkAllocationCallbacks* pAllocator);
//typedef void (*PFN_vkDebugReportMessageEXT)(VkInstance instance, VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT objectType, u64 object, usize location, i32 messageCode, const char* pLayerPrefix, const char* pMessage);
//
//#ifndef VK_NO_PROTOTYPES
//VkResult vkCreateDebugReportCallbackEXT(
//    VkInstance instance,
//    const VkDebugReportCallbackCreateInfoEXT* pCreateInfo,
//    const VkAllocationCallbacks* pAllocator,
//    VkDebugReportCallbackEXT* pCallback);
//
//void vkDestroyDebugReportCallbackEXT(
//    VkInstance instance,
//    VkDebugReportCallbackEXT callback,
//    const VkAllocationCallbacks* pAllocator);
//
//void vkDebugReportMessageEXT(
//    VkInstance instance,
//    VkDebugReportFlagsEXT flags,
//    VkDebugReportObjectTypeEXT objectType,
//    u64 object,
//    usize location,
//    i32 messageCode,
//    const char* pLayerPrefix,
//    const char* pMessage);
//#endif
//
//#endif
