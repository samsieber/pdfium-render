//! Defines the [PdfFormRadioButtonField] struct, exposing functionality related to a single
//! form field of type `PdfFormFieldType::RadioButton`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::form_field_private::internal::PdfFormFieldPrivate;

/// A single `PdfFormField` of type `PdfFormFieldType::RadioButton`. The form field object defines
/// an interactive radio button widget that can be toggled by the user.
///
/// Form fields in Pdfium are wrapped inside page annotations of type `PdfPageAnnotationType::Widget`
/// or `PdfPageAnnotationType::XfaWidget`. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// `PdfForm::field_values()` function.
pub struct PdfFormRadioButtonField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormRadioButtonField<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormRadioButtonField {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormRadioButtonField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns `true` if this [PdfFormRadioButtonField] object has its radio button selected.
    #[inline]
    pub fn is_checked(&self) -> Result<bool, PdfiumError> {
        self.is_checked_impl()
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormRadioButtonField<'a> {
    #[inline]
    fn form_handle(&self) -> &FPDF_FORMHANDLE {
        &self.form_handle
    }

    #[inline]
    fn annotation_handle(&self) -> &FPDF_ANNOTATION {
        &self.annotation_handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}